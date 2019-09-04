use std::collections::HashMap;

use crate::Acc;

pub fn show(data: HashMap<&str, Acc>) {
    dbg!(&data);

    for (dir, acc) in &data {
        let total = acc.get_total_bytes();

        for age in acc.get_ages() {
            let accessed = acc.get_accessed_bytes(*age).unwrap();
            let modified = acc.get_modified_bytes(*age).unwrap();

            println!("{}:{}:{}:{}:{}", age, total, accessed, modified, dir);
        }
    }
}
