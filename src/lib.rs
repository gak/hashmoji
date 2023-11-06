mod generated;

use crate::generated::EMOJIS;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct Hashmoji;

impl Hashmoji {
    pub fn hash(s: impl Hash) -> &'static str {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let hash = hasher.finish();

        EMOJIS[(hash % EMOJIS.len() as u64) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brainstorm() {
        println!("{}", Hashmoji::hash("gak"));
        println!("{}", Hashmoji::hash(123));
        println!("{}", Hashmoji::hash(vec!["a", "b", "c"]));
        println!("{}", Hashmoji::hash(true));
        println!("{}", Hashmoji::hash(false));
    }
}
