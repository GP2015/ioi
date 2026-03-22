use crate::state::State;
use ahash::AHashMap;

type Key = (State, u32);
type Value = State;

pub struct Cache {
    cache: AHashMap<Key, Value>,
    total: usize,
    hits: usize,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            cache: AHashMap::new(),
            total: 0,
            hits: 0,
        }
    }

    pub fn contains_key(&mut self, key: &Key) -> bool {
        self.total += 1;
        self.cache.contains_key(key)
    }

    pub fn get(&mut self, key: &Key) -> Option<&Value> {
        self.hits += 1;
        self.cache.get(key)
    }

    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.cache.insert(key, value)
    }

    // pub fn print_summary(&self) {
    //     println!(
    //         "Cache ({}): {} / {}",
    //         self.cache.len(),
    //         self.hits,
    //         self.total
    //     )
    // }
}
