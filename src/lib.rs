//! hashmoji is a crate that generates unique emojis based on the hash of a value.
//!
//! This is useful for generating unique emojis for a given value, such as a user ID, or a file
//! hash, instead of looking at a random UUID.
//!
//! hashmoji relies on the [Hash] trait to generate a unique emoji for a given value, which many
//! types implement, and of course you can implement it for your own types using `#[derive(Hash)]`.
//!
//! # Usage
//!
//! ```rust
//! let uuid = "30d8c256-0ffa-4e1b-8e1e-437bb0a0b45a";
//!
//! // Generate a single emoji:
//! let emoji = hashmoji::one(uuid);
//! # println!("{}", emoji);
//! assert_eq!(emoji, "🌤️");
//!
//! // You can also generate a fixed length string:
//! let emoji = hashmoji::fixed(uuid, 10);
//! # println!("{}", emoji);
//! assert_eq!(emoji, "🌤️♎😟🟩⤵️🇽🇰🇧🇦🏉🤠🦵");
//!
//! // A variable length string, depending on the hash:
//! let emoji = hashmoji::variable(uuid, 3..7);
//! # println!("{}", emoji);
//! assert_eq!(emoji, "🌤️♎😟🟩");
//! ```
//!
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![deny(unreachable_pub)]
#![deny(non_ascii_idents)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod generated;

use crate::generated::EMOJIS;
use core::hash::{Hash, Hasher};
use twox_hash::XxHash;

#[cfg(feature = "alloc")]
pub use alloc::string::String;

fn hasher() -> impl Hasher {
    XxHash::with_seed(0)
}

/// Generate one emoji based on the hash of the given value.
pub fn one(hashable: impl Hash) -> &'static str {
    let mut hasher = hasher();
    hashable.hash(&mut hasher);
    let hash = hasher.finish();

    EMOJIS[(hash % EMOJIS.len() as u64) as usize]
}

/// Choose a string size between the given range and generate a unique string of emojis.
#[cfg(feature = "alloc")]
pub fn variable(hashable: impl Hash, range: core::ops::Range<usize>) -> alloc::string::String {
    let mut hasher = crate::hasher();
    hashable.hash(&mut hasher);
    let hash = hasher.finish();

    let count = (hash % (range.end - range.start) as u64) as usize + range.start;

    fixed(hashable, count)
}

/// Generate a unique fixed length string of emojis based on the hash of the given value.
#[cfg(feature = "alloc")]
pub fn fixed(hashable: impl Hash, count: usize) -> String {
    let mut hasher = crate::hasher();
    let mut output = String::with_capacity(count);
    for _ in 0..count {
        // Hash it again for each emoji to make it more unique.
        hashable.hash(&mut hasher);
        let hash = hasher.finish();

        let emoji = EMOJIS[(hash % EMOJIS.len() as u64) as usize];
        output.push_str(emoji);
    }

    output
}

/// Iterate over all compiled emojis.
pub fn iter() -> impl Iterator<Item = &'static str> {
    EMOJIS.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use unicode_segmentation::UnicodeSegmentation;

    fn len(s: &str) -> usize {
        UnicodeSegmentation::graphemes(s, true).count()
    }

    #[test]
    fn test_one() {
        for a in 0..1000 {
            let s = one(a);
            assert_eq!(len(s), 1, "{}", s);
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_variable_length() {
        for a in 0..1000 {
            let s = variable(a, 1..10);
            assert!(len(&s) >= 1, "{}", s);
            assert!(len(&s) <= 10, "{}", s);
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_fixed_length() {
        for a in 0..1000 {
            let s = fixed(a, 10);
            assert_eq!(len(&s), 10, "{}", s);
        }
    }
}
