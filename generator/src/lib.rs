use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct Emoji<'a> {
    pub emoji: &'a str,
    pub group: String,
    pub subgroup: String,
    pub version: String,
}

pub struct Collection<'a> {
    pub emojis: Vec<Emoji<'a>>,
    pub group_features: Vec<String>,
    pub subgroup_features: Vec<String>,
}

impl Collection<'_> {
    pub fn all() -> Self {
        let mut emojis = Vec::with_capacity(4_000);
        let mut group_features = Vec::with_capacity(10);
        let mut subgroup_features = Vec::with_capacity(50);
        let mut group = "".to_string();
        let mut subgroup = "".to_string();

        let emoji = include_str!("../../emoji-test.txt");
        for line in emoji.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with("# group:") {
                let found_group = line.split(": ").nth(1).unwrap();
                group = to_feature_name(found_group);
                if group == "component".to_string() {
                    continue;
                }
                group_features.push(group.clone());
                continue;
            }

            if line.starts_with("# subgroup:") {
                let found_subgroup = line.split(": ").nth(1).unwrap();
                if group == "component".to_string() {
                    continue;
                }
                subgroup = to_feature_name(found_subgroup);
                subgroup_features.push(subgroup.clone());
                continue;
            }

            if line.starts_with("#") {
                continue;
            }

            if !line.contains("fully-qualified") {
                continue;
            }

            if group == "component" {
                continue;
            }

            // Grab the emoji and version after "# "
            let emoji = line.split("# ").nth(1).unwrap();
            let mut parts = emoji.split(" ");
            let emoji = parts.next().unwrap();
            let version = parts.next().unwrap();
            let version = version.replace("E", "");

            emojis.push(Emoji {
                group: group.clone(),
                subgroup: subgroup.clone(),
                version,
                emoji,
            });
        }

        Self {
            emojis,
            group_features,
            subgroup_features,
        }
    }
}

/// Open up Cargo.toml and find [features], then replace the rest.
pub fn write_features(path: &PathBuf, group_features: &[String], subgroup_features: &[String]) {
    let mut cargo = File::open(&path).unwrap();
    let mut cargo_toml = String::new();
    cargo.read_to_string(&mut cargo_toml).unwrap();

    let features_section = "[features]";
    let splits = cargo_toml.split(features_section).collect::<Vec<_>>();
    let content = splits[0];

    let mut output = Vec::with_capacity(100);
    output.push(features_section.to_string());
    output.push(r#"# The family feature is skipped by default because it has a lot of similar looking emojis,"#.to_string());
    output.push(r#"# so they appear more and become difficult to differentiate."#.to_string());
    output.push(r#"default = ["std", "family"]"#.to_string());
    output.push(r#"std = ["alloc"]"#.to_string());
    output.push(r#"alloc = []"#.to_string());
    output.push(r#"# Additive will only include emojis that have a feature enabled, as opposed to using removing them."#.to_string());
    output.push(r#"additive = []"#.to_string());

    output.push("# Group features".to_string());
    for feature in group_features {
        output.push(format!("{feature} = []"));
    }
    output.push("# Subgroup features".to_string());
    for feature in subgroup_features {
        output.push(format!("{feature} = []"));
    }

    let output = output.join("\n");

    let mut cargo_toml = File::create(&path).unwrap();
    cargo_toml.write_all(content.as_bytes()).unwrap();
    cargo_toml.write_all(output.as_bytes()).unwrap();
    cargo_toml.write_all("\n".as_bytes()).unwrap();

    let count = (group_features.len() + subgroup_features.len()) * 2;
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
            let include = has_env_feature(&emoji.group) || has_env_feature(&emoji.subgroup);

            if additive {
                include
            } else {
                !include
            }
        })
        .map(|emoji| emoji.emoji)
}
