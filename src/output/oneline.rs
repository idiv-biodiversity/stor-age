use std::collections::HashMap;
use std::hash::BuildHasher;

use crate::Data;

pub fn show<S: BuildHasher>(data: &HashMap<&str, Data, S>) {
    for (dir, data) in data {
        let t_b = data.get_total_bytes();
        let t_f = data.get_total_files();

        for age in data.get_ages() {
            let a_b = data.get_accessed_bytes(*age).unwrap_or_default();
            let m_b = data.get_modified_bytes(*age).unwrap_or_default();
            let a_f = data.get_accessed_files(*age).unwrap_or_default();
            let m_f = data.get_modified_files(*age).unwrap_or_default();

            println!("{age}:{t_b}:{a_b}:{m_b}:{t_f}:{a_f}:{m_f}:{dir}");
        }
    }
}
