use std::collections::HashMap;

use crate::Acc;

pub fn show(data: HashMap<&str, Acc>) {
    println!("# HELP stor_age_bytes_total Total size.");
    println!("# TYPE stor_age_bytes_total gauge");

    for (dir, acc) in &data {
        println!(
            "stor_age_bytes_total{{dir=\"{}\"}} {}",
            dir,
            acc.get_total_bytes()
        )
    }

    println!();
    println!("# HELP stor_age_bytes_accessed Accessed size.");
    println!("# TYPE stor_age_bytes_accessed gauge");

    for (dir, acc) in &data {
        for age in acc.get_ages() {
            println!(
                "stor_age_bytes_accessed{{dir=\"{}\",age=\"{}\"}} {}",
                dir,
                age,
                acc.get_accessed_bytes(*age).unwrap()
            )
        }
    }

    println!();
    println!("# HELP stor_age_bytes_modified Modified size.");
    println!("# TYPE stor_age_bytes_modified gauge");

    for (dir, acc) in &data {
        for age in acc.get_ages() {
            println!(
                "stor_age_bytes_modified{{dir=\"{}\",age=\"{}\"}} {}",
                dir,
                age,
                acc.get_modified_bytes(*age).unwrap()
            )
        }
    }
}
