use bytesize::ByteSize;
use clap::arg_enum;

use crate::Acc;

arg_enum! {
    #[derive(Clone, Copy)]
    pub enum Output {
        Pretty,
        Oneline,
    }
}

pub fn pretty(dir: &str, acc: Acc) {
    let total = acc.get_total_bytes();

    let t_b = ByteSize(total).to_string_as(true);
    println!("{}: {}", dir, t_b);

    for age in acc.get_ages() {
        let accessed = acc.get_accessed_bytes(*age).unwrap();
        let modified = acc.get_modified_bytes(*age).unwrap();

        let (a_p, m_p) = if total == 0 {
            (0.0, 0.0)
        } else {
            let a_p = ((accessed as f64) / (total as f64) * 100.0).round();
            let m_p = ((modified as f64) / (total as f64) * 100.0).round();
            (a_p, m_p)
        };

        let a_b = ByteSize(accessed).to_string_as(true);
        let m_b = ByteSize(modified).to_string_as(true);

        println!("unaccessed for {} days: {}% ({})", age, a_p, a_b,);
        println!("unmodified for {} days: {}% ({})", age, m_p, m_b,);
    }
}

pub fn oneline(dir: &str, acc: Acc) {
    let total = acc.get_total_bytes();

    for age in acc.get_ages() {
        let accessed = acc.get_accessed_bytes(*age).unwrap();
        let modified = acc.get_modified_bytes(*age).unwrap();

        println!("{}:{}:{}:{}:{}", age, total, accessed, modified, dir);
    }
}
