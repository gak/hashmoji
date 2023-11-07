use hashmoji_generator::{write_features, Collection};
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let mut cargo_path = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    cargo_path.push("..");
    cargo_path.push("Cargo.toml");

    let collection = Collection::all();
    write_features(&cargo_path, collection);
}
