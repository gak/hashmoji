use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let mut path = PathBuf::from_str(&env::var("OUT_DIR").unwrap()).unwrap();
    path.push("generated.rs");

    let collection = hashmoji_generator::Collection::all();
    let filtered = hashmoji_generator::filter(&collection);
    write_emojis(&path, filtered);
}

fn write_emojis<'a>(path: &PathBuf, emojis: impl Iterator<Item = &'a str>) {
    let emojis = emojis.collect::<Vec<_>>();
    let count = emojis.len();

    let mut generated = File::create(path).unwrap();
    generated
        .write_all(format!("pub(crate) const EMOJIS: [&str; {count}] = [\n").as_bytes())
        .unwrap();

    for emoji in emojis {
        generated
            .write_all(format!("\"{}\",\n", emoji).as_bytes())
            .unwrap();
    }

    generated.write_all(b"];\n").unwrap();

    eprintln!("Wrote {count} emojis to {path:?}");
}
