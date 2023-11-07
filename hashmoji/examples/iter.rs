//! This example is just to dump all the compiled emojis.
//!
//! Specify `--lines` to print one emoji per line.
//!
//! Used for testing, seeing how they appear in your IDE/console, or to see if your feature set is
//! to your liking.

#[cfg(not(feature = "std"))]
compile_error!("This example requires std");

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.contains(&"--lines".to_string()) {
        for emoji in hashmoji::iter() {
            println!("{}", emoji);
        }
        return;
    }

    println!("{}", hashmoji::iter().collect::<String>());
    eprintln!("{}", hashmoji::iter().count());
}
