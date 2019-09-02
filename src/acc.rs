use std::collections::HashMap;
use std::ops::AddAssign;

#[derive(Debug, Default)]
struct AccInternal {
    accessed: u64,
    modified: u64,
}

impl AccInternal {
    fn new(accessed: u64, modified: u64) -> AccInternal {
        AccInternal { accessed, modified }
    }
}

impl AddAssign for AccInternal {
    fn add_assign(&mut self, other: Self) {
        *self = AccInternal {
            accessed: self.accessed + other.accessed,
            modified: self.modified + other.modified,
        }
    }
}

#[derive(Debug, Default)]
pub struct Acc {
    total: u64,
    data: HashMap<u64, AccInternal>,
}

impl Acc {
    pub fn new() -> Acc {
        Default::default()
    }

    pub fn with_total(mut self, total: u64) -> Acc {
        self.total = total;
        self
    }

    pub fn get_accessed_bytes(&self, age: u64) -> Option<u64> {
        self.data.get(&age).map(|data| data.accessed)
    }

    pub fn get_modified_bytes(&self, age: u64) -> Option<u64> {
        self.data.get(&age).map(|data| data.modified)
    }

    pub fn get_total_bytes(&self) -> u64 {
        self.total
    }

    pub fn get_ages(&self) -> Vec<&u64> {
        let mut ages: Vec<&u64> = self.data.keys().collect();
        ages.sort();
        ages
    }

    pub fn insert(&mut self, age: u64, accessed: u64, modified: u64) {
        let a = AccInternal::new(accessed, modified);
        self.data.insert(age, a);
    }
}

impl AddAssign for Acc {
    fn add_assign(&mut self, other: Acc) {
        self.total += other.total;

        for (age, acc) in other.data {
            let sum = self.data.entry(age).or_default();
            *sum += acc
        }
    }
}
