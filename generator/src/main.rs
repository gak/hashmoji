use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let emoji = include_str!("../../emoji-test.txt");

    let mut path = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    path.push("..");
    path.push("src");
    path.push("generated.rs");

    let mut collected = Vec::with_capacity(4_000);

    for line in emoji.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            continue;
        }

        if !line.contains("fully-qualified") {
            continue;
        }

        // Grab the emoji after "# "
        let emoji = line.split("# ").nth(1).unwrap();
        let emoji = emoji.split(" ").next().unwrap();

        collected.push(emoji);
    }

    let count = collected.len();
    let mut generated = File::create(&path).unwrap();
    generated.write_all(format!("pub(crate) const EMOJIS: [&str; {count}] = [").as_bytes()).unwrap();

    for (idx, emoji) in collected.iter().enumerate() {
        if idx % 10 == 0 {
            generated.write_all(b"\n    ").unwrap();
        }
        generated.write_all(format!("\"{}\", ", emoji).as_bytes()).unwrap();
    }

    generated.write_all(b"\n];\n").unwrap();

    println!("Wrote {count} emojis to {path:?}");
}
