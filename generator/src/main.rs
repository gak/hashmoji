use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let emoji = include_str!("../../emoji-test.txt");

    let mut base_path = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    base_path.push("..");

    let mut src_generated_path = base_path.clone();
    src_generated_path.push("src");
    src_generated_path.push("generated.rs");

    let mut cargo_path = base_path.clone();
    cargo_path.push("Cargo.toml");

    let mut collected = Vec::with_capacity(4_000);
    let mut features = Vec::with_capacity(50);
    let mut group = "Smileys & Emotion";
    let mut subgroup = "face-smiling";

    for line in emoji.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("# group:") {
            group = line.split(": ").nth(1).unwrap();
            let group = to_feature_name(group);
            println!("group {}", group);
            features.push(group);
            continue;
        }

        if line.starts_with("# subgroup:") {
            subgroup = line.split(": ").nth(1).unwrap();
            let subgroup = to_feature_name(subgroup);
            println!("subgroup {}", subgroup);
            features.push(subgroup);
            continue;
        }

        if line.starts_with("#") {
            continue;
        }

        if !line.contains("fully-qualified") {
            continue;
        }

        // We want unique looking emojis, however kiss, holding hands and couple have many
        // permutations. We don't want to include all of them, so we skip all of them except the
        // "base" ones.
        if line.contains("holding hands:") {
            continue;
        }
        if line.contains("couple with heart:") {
            continue;
        }
        if line.contains("kiss:") {
            continue;
        }

        // Grab the emoji and version after "# "
        let emoji = line.split("# ").nth(1).unwrap();
        let mut parts = emoji.split(" ");
        let emoji = parts.next().unwrap();
        let version = parts.next().unwrap();

        collected.push((group, subgroup, version, emoji));
    }

    write_emojis(&src_generated_path, collected);

    write_features(&cargo_path, &features);
}

fn write_emojis(path: &PathBuf, collected: Vec<(&str, &str, &str, &str)>) {
    let count = collected.len();
    let mut generated = File::create(&path).unwrap();
    generated
        .write_all(format!("pub(crate) const EMOJIS: [&str; {count}] = [").as_bytes())
        .unwrap();

    for (idx, (group, subgroup, version, emoji)) in collected.into_iter().enumerate() {
        // generated.write_all().unwrap();
        generated
            .write_all(format!("\"{}\",\n", emoji).as_bytes())
            .unwrap();
    }

    generated.write_all(b"\n];\n").unwrap();

    println!("Wrote {count} emojis to {path:?}");
}

/// Open up Cargo.toml and find [features], then replace the rest.
fn write_features(path: &PathBuf, features: &[String]) {
    let mut cargo = File::open(&path).unwrap();
    let mut cargo_toml = String::new();
    cargo.read_to_string(&mut cargo_toml).unwrap();

    let features_section = "[features]";
    let splits = cargo_toml.split(features_section).collect::<Vec<_>>();
    let content = splits[0].trim();

    let mut output = Vec::with_capacity(100);
    output.push(features_section.to_string());
    output.push(r#"default = ["skip-family"]"#.to_string());

    for feature in features {
        output.push(format!("{feature} = []"));
        output.push(format!("skip-{feature} = []"));
    }

    let output = output.join("\n");

    let mut cargo_toml = File::create(&path).unwrap();
    cargo_toml.write_all(content.as_bytes()).unwrap();
    cargo_toml.write_all(output.as_bytes()).unwrap();

    let count = features.len();
    println!("Wrote {count} features to {path:?}");
}

/// Convert a string to a valid rust feature identifier.
fn to_feature_name(s: &str) -> String {
    let mut s = s.to_lowercase();
    s = s.replace(" ", "-");
    s = s.replace("&", "and");
    s
}
