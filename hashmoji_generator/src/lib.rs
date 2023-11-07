use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct Emoji<'a> {
    pub emoji: &'a str,
    pub group: String,
    pub subgroup: String,
    pub version: u16, // 15.1 = 1510
    pub has_gender: bool,
    pub has_skin_tone: bool,
    pub has_hair_style: bool,
}

pub struct Collection<'a> {
    pub emojis: Vec<Emoji<'a>>,
    pub group_features: Vec<String>,
    pub subgroup_features: Vec<String>,
    pub versions: HashSet<u16>,
}

impl Collection<'_> {
    #[allow(clippy::nonminimal_bool)]
    pub fn all() -> Self {
        let mut emojis = Vec::with_capacity(4_000);
        let mut group_features = Vec::with_capacity(10);
        let mut subgroup_features = Vec::with_capacity(50);
        let mut versions = HashSet::with_capacity(10);
        let mut group = "".to_string();
        let mut subgroup = "".to_string();

        let emoji = include_str!("../emoji-test.txt");
        for line in emoji.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with("# group:") {
                let found_group = line.split(": ").nth(1).unwrap();
                group = to_feature_name(found_group);
                if &group == "component" {
                    continue;
                }
                group_features.push(group.clone());
                continue;
            }

            if line.starts_with("# subgroup:") {
                let found_subgroup = line.split(": ").nth(1).unwrap();
                if &group == "component" {
                    continue;
                }
                subgroup = to_feature_name(found_subgroup);
                subgroup_features.push(subgroup.clone());
                continue;
            }

            if line.starts_with('#') {
                continue;
            }

            if !line.contains("fully-qualified") {
                continue;
            }

            if group == "component" {
                continue;
            }

            if group.is_empty() || subgroup.is_empty() {
                panic!("Missing group or subgroup");
            }

            // Grab the emoji and version after "# "
            let emoji = line.split("# ").nth(1).unwrap();
            let mut parts = emoji.split(' ');
            let emoji = parts.next().unwrap();
            let has_gender = false
                || emoji.contains('\u{2640}')
                || emoji.contains('\u{2642}')
                || emoji.contains('\u{1F468}')
                || emoji.contains('\u{1F469}');
            let has_skin_tone = false
                || emoji.contains('🏻')
                || emoji.contains('🏼')
                || emoji.contains('🏽')
                || emoji.contains('🏾')
                || emoji.contains('🏿');
            let has_hair = false
                || emoji.contains('🦰')
                || emoji.contains('🦱')
                || emoji.contains('🦳')
                || emoji.contains('🦲');

            let version = parts.next().unwrap();
            let version = version.replace('E', "");

            // Convert version to u16, i.e. "15.1" -> 1501
            let parts = version.split('.').collect::<Vec<_>>();
            let major = parts[0].parse::<u16>().unwrap();
            let minor = parts[1].parse::<u16>().unwrap();
            let version = major * 100 + minor;
            versions.insert(version);

            emojis.push(Emoji {
                group: group.clone(),
                subgroup: subgroup.clone(),
                version,
                emoji,
                has_gender,
                has_skin_tone,
                has_hair_style: has_hair,
            });
        }

        Self {
            emojis,
            group_features,
            subgroup_features,
            versions,
        }
    }
}

/// Open up Cargo.toml and find [features], then replace the rest.
pub fn write_features(path: &PathBuf, collection: Collection) {
    let mut cargo = File::open(path).unwrap();
    let mut cargo_toml = String::new();
    cargo.read_to_string(&mut cargo_toml).unwrap();

    let features_section = "[features]";
    let splits = cargo_toml.split(features_section).collect::<Vec<_>>();
    let content = splits[0];

    let mut output = Vec::with_capacity(100);
    output.push(features_section.to_string());
    output.push(r#"# Modifiers are skipped by default because it has a lot of similar looking emojis (depending on the font of course)."#.to_string());
    output.push(r#"default = ["std", "all-modifiers"]"#.to_string());
    output.push(r#"std = ["alloc"]"#.to_string());
    output.push(r#"alloc = []"#.to_string());
    output.push(r#"derive = ["dep:hashmoji_derive"]"#.to_string());
    output.push(r#"# Additive will only include emojis that have a feature enabled, as opposed to using removing them."#.to_string());
    output.push(r#"additive = []"#.to_string());
    output.push(r#"all-modifiers = ["skin-tones", "genders", "hair-styles"]"#.to_string());
    output.push(r#"skin-tones = []"#.to_string());
    output.push(r#"genders = []"#.to_string());
    output.push(r#"hair-styles = []"#.to_string());

    output.push("# Group features".to_string());
    for feature in &collection.group_features {
        output.push(format!("{feature} = []"));
    }

    output.push("# Subgroup features".to_string());
    for feature in &collection.subgroup_features {
        output.push(format!("{feature} = []"));
    }

    output.push("# Maximum unicode version to support (v1501 = v15.1)".to_string());
    let mut versions = collection.versions.iter().collect::<Vec<_>>();
    versions.sort();
    for version in versions {
        output.push(format!("{} = []", to_feature_version(*version)));
    }

    let output = output.join("\n");

    let mut cargo_toml = File::create(path).unwrap();
    cargo_toml.write_all(content.as_bytes()).unwrap();
    cargo_toml.write_all(output.as_bytes()).unwrap();
    cargo_toml.write_all("\n".as_bytes()).unwrap();

    println!(
        "Wrote {} group features, {} subgroup features, {} version features to {path:?}",
        collection.group_features.len(),
        collection.subgroup_features.len(),
        collection.versions.len(),
    );
}

fn has_env_feature(s: &str) -> bool {
    let mut s = s.to_uppercase();
    s = s.replace('-', "_");
    s = format!("CARGO_FEATURE_{}", s);
    env::var(s).is_ok()
}

/// Convert a string to a valid rust feature identifier.
fn to_feature_name(s: &str) -> String {
    let mut s = s.to_lowercase();
    s = s.replace(' ', "-");
    s = s.replace('&', "and");
    s
}

fn to_feature_version(version: u16) -> String {
    format!("v{:04}", version)
}

/// Filter out emojis based on features (via build.rs CARGO_FEATURE_*).
pub fn filter<'a>(collection: &'a Collection<'a>) -> impl Iterator<Item = &'a str> {
    let additive = has_env_feature("additive");
    let skin_tones = has_env_feature("skin-tones");
    let genders = has_env_feature("genders");
    let hair_styles = has_env_feature("hair-styles");

    let versions: Vec<u16> = env::vars()
        .filter_map(|(k, _)| {
            if !k.starts_with("CARGO_FEATURE_V") {
                return None;
            }

            let version = k.replace("CARGO_FEATURE_V", "");
            let version = version.parse::<u16>().unwrap();
            Some(version)
        })
        .collect();
    if versions.len() > 1 {
        panic!("Only one version feature can be enabled at a time.");
    }

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
        .filter(move |emoji| {
            if let Some(version) = versions.first() {
                emoji.version <= *version
            } else {
                true
            }
        })
        .filter(move |emoji| filterable(additive, skin_tones, emoji.has_skin_tone))
        .filter(move |emoji| filterable(additive, genders, emoji.has_gender))
        .filter(move |emoji| filterable(additive, hair_styles, emoji.has_hair_style))
        .map(|emoji| emoji.emoji)
}

fn filterable(additive: bool, feature_flag: bool, has_emoji_modifier: bool) -> bool {
    // e.g. doesn't have a hair style, we always include it.
    if !has_emoji_modifier {
        return true;
    }

    if additive {
        // We're adding to the set, so only include if the feature is enabled.
        feature_flag
    } else {
        // We're subtracting from the set, so only include if the feature is disabled.
        !feature_flag
    }
}
