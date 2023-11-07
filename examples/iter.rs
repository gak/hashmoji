//! This example is just to dump all the compiled emojis.
//!
//! Used for testing, or to see if your feature set looks good.

#[cfg(not(feature = "std"))]
compile_error!("This example requires std");

fn main() {
    println!("{}", hashmoji::iter().collect::<String>());
}
