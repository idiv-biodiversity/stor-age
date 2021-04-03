#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod analysis;
mod config;
pub mod log;
mod output;

use std::collections::HashMap;
use std::ops::AddAssign;

pub use analysis::run;
pub use config::Config;
pub use config::Output;

#[derive(Debug, Default)]
struct Count {
    accessed_bytes: u64,
    modified_bytes: u64,
    accessed_files: u64,
    modified_files: u64,
}

impl AddAssign for Count {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            accessed_bytes: self.accessed_bytes + other.accessed_bytes,
            modified_bytes: self.modified_bytes + other.modified_bytes,
            accessed_files: self.accessed_files + other.accessed_files,
            modified_files: self.modified_files + other.modified_files,
        }
    }
}

#[derive(Debug, Default)]
pub struct Data {
    total_bytes: u64,
    total_files: u64,
    data: HashMap<u64, Count>,
}

impl Data {
    #[must_use]
    pub fn with_ages(mut self, ages: &[u64]) -> Self {
        for age in ages {
            self.insert(*age, 0, 0, 0, 0);
        }

        self
    }

    #[must_use]
    pub const fn with_total_bytes(mut self, bytes: u64) -> Self {
        self.total_bytes = bytes;
        self
    }

    #[must_use]
    pub const fn with_total_files(mut self, files: u64) -> Self {
        self.total_files = files;
        self
    }

    #[must_use]
    pub fn get_accessed_bytes(&self, age: u64) -> Option<u64> {
        self.data.get(&age).map(|data| data.accessed_bytes)
    }

    #[must_use]
    pub fn get_modified_bytes(&self, age: u64) -> Option<u64> {
        self.data.get(&age).map(|data| data.modified_bytes)
    }

    #[must_use]
    pub fn get_accessed_files(&self, age: u64) -> Option<u64> {
        self.data.get(&age).map(|data| data.accessed_files)
    }

    #[must_use]
    pub fn get_modified_files(&self, age: u64) -> Option<u64> {
        self.data.get(&age).map(|data| data.modified_files)
    }

    #[must_use]
    pub const fn get_total_bytes(&self) -> u64 {
        self.total_bytes
    }

    #[must_use]
    pub const fn get_total_files(&self) -> u64 {
        self.total_files
    }

    #[must_use]
    pub fn get_ages(&self) -> Vec<&u64> {
        let mut ages: Vec<&u64> = self.data.keys().collect();
        ages.sort();
        ages
    }

    pub fn insert(
        &mut self,
        age: u64,
        accessed_bytes: u64,
        modified_bytes: u64,
        accessed_files: u64,
        modified_files: u64,
    ) {
        let a = Count {
            accessed_bytes,
            modified_bytes,
            accessed_files,
            modified_files,
        };

        self.data.insert(age, a);
    }
}

impl AddAssign for Data {
    fn add_assign(&mut self, other: Self) {
        self.total_bytes += other.total_bytes;
        self.total_files += other.total_files;

        for (age, acc) in other.data {
            let sum = self.data.entry(age).or_default();
            *sum += acc
        }
    }
}
