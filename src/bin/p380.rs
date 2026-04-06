use std::collections::hash_map::{Entry, HashMap};

struct RandomizedSet {
    values: Vec<i32>,
    indices: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self { values: vec![], indices: HashMap::new() }
    }

    fn insert(&mut self, value: i32) -> bool {
        if let Entry::Vacant(entry) = self.indices.entry(value) {
            entry.insert(self.values.len());
            self.values.push(value);
            true
        } else {
            false
        }
    }

    fn remove(&mut self, value: i32) -> bool {
        if let Some(index) = self.indices.remove(&value) {
            self.values.swap_remove(index);
            self.values.get(index).map(|&value| self.indices.insert(value, index));
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        self.values[rand::random::<usize>() % self.values.len()]
    }
}

fn main() {
    let mut obj = RandomizedSet::new();
    assert_eq!(obj.insert(1), true);
    assert_eq!(obj.remove(2), false);
    assert_eq!(obj.insert(2), true);
    assert!(obj.indices.contains_key(&obj.get_random()));
    assert_eq!(obj.remove(1), true);
    assert_eq!(obj.insert(2), false);
    assert!(obj.indices.contains_key(&obj.get_random()));
}
