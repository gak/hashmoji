use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct Emoji<'a> {
    pub emoji: &'a str,
    pub group: &'a str,
    pub subgroup: &'a str,
    pub version: String,
}

pub struct Collection<'a> {
    pub emojis: Vec<Emoji<'a>>,
    pub features: Vec<String>,
}

impl Collection<'_> {
    pub fn all() -> Self {
        let mut emojis = Vec::with_capacity(4_000);
        let mut features = Vec::with_capacity(50);
        let mut group = "";
        let mut subgroup = "";

        let emoji = include_str!("../../emoji-test.txt");
        for line in emoji.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with("# group:") {
                group = line.split(": ").nth(1).unwrap();
                let group = to_feature_name(group);
                features.push(group);
                continue;
            }

            if line.starts_with("# subgroup:") {
                subgroup = line.split(": ").nth(1).unwrap();
                let subgroup = to_feature_name(subgroup);
                features.push(subgroup);
                continue;
            }

            if line.starts_with("#") {
                continue;
            }

            if !line.contains("fully-qualified") {
                continue;
            }

            // Grab the emoji and version after "# "
            let emoji = line.split("# ").nth(1).unwrap();
            let mut parts = emoji.split(" ");
            let emoji = parts.next().unwrap();
            let version = parts.next().unwrap();
            let version = version.replace("E", "");

            emojis.push(Emoji {
                group,
                subgroup,
                version,
                emoji,
            });
        }

        Self { emojis, features }
    }
}

/// Open up Cargo.toml and find [features], then replace the rest.
pub fn write_features(path: &PathBuf, features: &[String]) {
    let mut cargo = File::open(&path).unwrap();
    let mut cargo_toml = String::new();
    cargo.read_to_string(&mut cargo_toml).unwrap();

    let features_section = "[features]";
    let splits = cargo_toml.split(features_section).collect::<Vec<_>>();
    let content = splits[0];

    let mut output = Vec::with_capacity(100);
    output.push(features_section.to_string());
    output.push(r#"default = ["std", "skip-family"]"#.to_string());
    output.push(r#"std = ["alloc"]"#.to_string());
    output.push(r#"alloc = []"#.to_string());
    output.push(r#"additive = []"#.to_string());

    for feature in features {
        output.push(format!("{feature} = []"));
    }
    for feature in features {
        output.push(format!("skip-{feature} = []"));
    }

    let output = output.join("\n");

    let mut cargo_toml = File::create(&path).unwrap();
    cargo_toml.write_all(content.as_bytes()).unwrap();
    cargo_toml.write_all(output.as_bytes()).unwrap();
    cargo_toml.write_all("\n".as_bytes()).unwrap();

    let count = features.len() * 2;
    println!("Wrote {count} features to {path:?}");
}

fn has_env_feature(s: &str) -> bool {
    let mut s = s.to_uppercase();
    s = s.replace("-", "_");
    s = format!("CARGO_FEATURE_{}", s);
    env::var(s).is_ok()
}

/// Convert a string to a valid rust feature identifier.
pub fn to_feature_name(s: &str) -> String {
    let mut s = s.to_lowercase();
    s = s.replace(" ", "-");
    s = s.replace("&", "and");
    s
}

/// Filter out emojis based on group and subgroup features (via build.rs CARGO_FEATURE_*).
pub fn filter<'a>(collection: &'a Collection<'a>) -> impl Iterator<Item = &'a str> {
    // Additive means we only include emojis that have a feature enabled.
    // In this case, you can add a group, but skip a subgroup.
    //
    // When not additive, all emojis are included by default, and you can remove them using
    // skip- features. The "add" features still work if you want to skip a group, but still include
    // a subgroup.
    let additive = env::var("CARGO_FEATURE_ADDITIVE").is_ok();

    collection
        .emojis
        .iter()
        .filter(move |emoji| {
            let include = has_env_feature(emoji.group) || has_env_feature(emoji.subgroup);
            let skip = has_env_feature(&format!("skip-{}", emoji.group))
                || has_env_feature(&format!("skip-{}", emoji.subgroup));

            if additive {
                include && !skip
            } else {
                !skip
            }
        })
        .map(|emoji| emoji.emoji)
}
