use std::collections::HashMap;

use crate::Data;

pub fn show(data: &HashMap<&str, Data>) {
    show_bytes(data);
    println!();
    show_files(data);
}

fn show_bytes(data: &HashMap<&str, Data>) {
    println!("# HELP stor_age_bytes_total Total size in bytes.");
    println!("# TYPE stor_age_bytes_total gauge");

    for (dir, data) in data {
        println!(
            "stor_age_bytes_total{{dir=\"{}\"}} {}",
            dir,
            data.get_total_bytes()
        )
    }

    println!();
    println!("# HELP stor_age_bytes_accessed Accessed size in bytes.");
    println!("# TYPE stor_age_bytes_accessed gauge");

    for (dir, data) in data {
        for age in data.get_ages() {
            println!(
                "stor_age_bytes_accessed{{dir=\"{}\",age=\"{}\"}} {}",
                dir,
                age,
                data.get_accessed_bytes(*age).unwrap()
            )
        }
    }

    println!();
    println!("# HELP stor_age_bytes_modified Modified size in bytes.");
    println!("# TYPE stor_age_bytes_modified gauge");

    for (dir, data) in data {
        for age in data.get_ages() {
            println!(
                "stor_age_bytes_modified{{dir=\"{}\",age=\"{}\"}} {}",
                dir,
                age,
                data.get_modified_bytes(*age).unwrap()
            )
        }
    }
}

fn show_files(data: &HashMap<&str, Data>) {
    println!("# HELP stor_age_files_total Total number of files.");
    println!("# TYPE stor_age_files_total gauge");

    for (dir, data) in data {
        println!(
            "stor_age_files_total{{dir=\"{}\"}} {}",
            dir,
            data.get_total_files()
        )
    }

    println!();
    println!("# HELP stor_age_files_accessed Accessed number of files.");
    println!("# TYPE stor_age_files_accessed gauge");

    for (dir, data) in data {
        for age in data.get_ages() {
            println!(
                "stor_age_files_accessed{{dir=\"{}\",age=\"{}\"}} {}",
                dir,
                age,
                data.get_accessed_files(*age).unwrap()
            )
        }
    }

    println!();
    println!("# HELP stor_age_files_modified Modified number of files.");
    println!("# TYPE stor_age_files_modified gauge");

    for (dir, data) in data {
        for age in data.get_ages() {
            println!(
                "stor_age_files_modified{{dir=\"{}\",age=\"{}\"}} {}",
                dir,
                age,
                data.get_modified_files(*age).unwrap()
            )
        }
    }
}
