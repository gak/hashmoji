//! hashmoji generates deterministic emoji based on the hash of a value.
//!
//! This is useful for generating unique emojis for a given value, such as a user ID, a file
//! hash, UUID, etc. It is easier for the eye to recognise an emoji than remembering some random
//! alphanumeric strings.
//!
//! hashmoji relies on the `Hash` trait to generate a unique emoji for a given value, which many
//! types implement, and of course you can implement it for your own types using `#[derive(Hash)]`.
//!
//! # Capabilities
//!
//! - Generates emoji from a value:
//!   - [`hashmoji::single()`](https://docs.rs/hashmoji/latest/hashmoji/fn.one.html)
//!   - [`hashmoji::fixed()`](https://docs.rs/hashmoji/latest/hashmoji/fn.fixed.html)
//!   - [`hashmoji::variable()`](https://docs.rs/hashmoji/latest/hashmoji/fn.variable.html)
//! * Supports multiple versions of Unicode up to 15.1.
//! * Supports `no_std` and optionally with `alloc`.
//! * Choose the set of emojis to be selected from via groups, subgroups, modifiers, version, etc.
//! * Generates configurable emoji sets during build so there's minimal runtime overhead.
//!
//! # Usage
//!
//! ```rust
//! let uuid = "30d8c256-0ffa-4e1b-8e1e-437bb0a0b45a";
//!
//! // Generate a single emoji:
//! let emoji = hashmoji::one(uuid);
//! assert_eq!(emoji, "🌤️");
//!
//! // You can also generate a fixed length string:
//! let emoji = hashmoji::fixed(uuid, 10);
//! assert_eq!(emoji, "🌤️♎😟🟩⤵️🇽🇰🇧🇦🏉🤠🦵");
//!
//! // A variable length string, depending on the hash:
//! let emoji = hashmoji::variable(uuid, 3..7);
//! assert_eq!(emoji, "🌤️♎😟🟩");
//! ```
//!
//! # Features
//!
//! - `std` (default): Enables the use of `std` types and functionality.
//! - `alloc` (default): Enables the use of `alloc` types and functionality. Having this off will remove `fixed()` and `variable()`.
//! - `additive`: Add to an empty set, instead of removing from the full set of emojis.
//!
//! # Filtering Features
//!
//! By default, filtering features remove from the full set of emoji. When enabling the `additive`
//! feature, the set of emojis start empty and the filtering features add to the set.
//!
//! Also by default, the `all-modifiers` filter is enabled because there are so many very similar
//! permutations of the same emoji where it becomes difficult to differentiate them.
//!
//! ## Groups and Subgroups
//!
//! You can filter by groups and subgroups too. Here are all the groups:
//!
//! - `smileys-and-emotion`
//! - `people-and-body`
//! - `animals-and-nature`
//! - `food-and-drink`
//! - `travel-and-places`
//! - `activities`
//! - `objects`
//! - `symbols`
//! - `flags`
//!
//! And some of the subgroups. See [Cargo.toml](Cargo.toml) for the full list:
//!
//! - `face-smiling`
//! - `face-affection`
//! - `face-tongue`
//! - `face-hand`
//! - `face-neutral-skeptical`
//! - `face-sleepy`
//! - etc...
//!
//! ## Modifiers
//!
//! - `all-modifiers` (default): Filter hair-styles, skin-tones, genders.
//! - `hair-styles`: Filter hair-styles.
//! - `skin-tones`: Filter skin-tones.
//! - `genders`: Filter gender modifiers.
//!
//! ## Versions
//!
//! You can choose the maximum Unicode version to be used. By default this is not set, so all versions are used. Versions are two digits for major, two for minor, e.g. `v1501` = `15.1`.
//!
//! Supported versions are `v1510`, `v1500`, `v1400`, `v1301`, `v1300`, `v1201`, `v1200`, `v1100`, `v0500`, `v0400`, `v0300`, `v0200`, `v0100`, `v0007`, `v0006`, `v0600`.
//!
//! # License
//!
//! **hashmoji** is licensed under either the [MIT license](LICENSE-MIT) or
//! the [Apache-2.0 license](LICENSE-APACHE), at your discretion.
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

use core::hash::{Hash, Hasher};
use twox_hash::XxHash;

#[cfg(feature = "alloc")]
pub use alloc::string::String;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn hasher() -> impl Hasher {
    XxHash::with_seed(0)
}

///! Generate one emoji based on the hash of the given value.
pub fn one(hashable: impl Hash) -> &'static str {
    let mut hasher = hasher();
    hashable.hash(&mut hasher);
    let hash = hasher.finish();

    EMOJIS[(hash % EMOJIS.len() as u64) as usize]
}

///! Choose a string size between the given range and generate a unique string of emojis.
#[cfg(feature = "alloc")]
pub fn variable(hashable: impl Hash, range: core::ops::Range<usize>) -> alloc::string::String {
    let mut hasher = crate::hasher();
    hashable.hash(&mut hasher);
    let hash = hasher.finish();

    let count = (hash % (range.end - range.start) as u64) as usize + range.start;

    fixed(hashable, count)
}

///! Generate a unique fixed length string of emojis based on the hash of the given value.
#[cfg(feature = "alloc")]
pub fn fixed(hashable: impl Hash, count: usize) -> String {
    let mut hasher = crate::hasher();
    let mut output = String::with_capacity(count);
    for _ in 0..count {
        //! Hash it again for each emoji to make it more unique.
        hashable.hash(&mut hasher);
        let hash = hasher.finish();

        let emoji = EMOJIS[(hash % EMOJIS.len() as u64) as usize];
        output.push_str(emoji);
    }

    output
}

///! Iterate over all compiled emojis.
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
