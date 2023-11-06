use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::generated::EMOJIS;

mod generated;

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
        println!("{}", Hashmoji::hash(0));
        println!("{}", Hashmoji::hash(1));
        println!("{}", Hashmoji::hash(2));
    }
}