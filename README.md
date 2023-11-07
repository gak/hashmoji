# hashmoji

hashmoji is a crate that generates deterministic unique emojis based on the hash of a value.

This is useful for generating unique emojis for a given value, such as a user ID, or a file
hash, instead of looking at a random UUID.

hashmoji relies on the `Hash` trait to generate a unique emoji for a given value, which many
types implement, and of course you can implement it for your own types using `#[derive(Hash)]`.

# Usage

```rust
let uuid = "30d8c256-0ffa-4e1b-8e1e-437bb0a0b45a";

// Generate a single emoji:
let emoji = hashmoji::one(uuid);
assert_eq!(emoji, "🌤️");

// You can also generate a fixed length string:
let emoji = hashmoji::fixed(uuid, 10);
assert_eq!(emoji, "🌤️♎😟🟩⤵️🇽🇰🇧🇦🏉🤠🦵");

// A variable length string, depending on the hash:
let emoji = hashmoji::variable(uuid, 3..7);
assert_eq!(emoji, "🌤️♎😟🟩");
```

## License

**hashmoji** is licensed under either the [MIT license](LICENSE-MIT) or
the [Apache-2.0 license](LICENSE-APACHE), at your discretion.
