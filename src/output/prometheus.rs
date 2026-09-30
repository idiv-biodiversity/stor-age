use std::collections::BTreeMap;
use std::io::{self, Write};

use crate::Data;

/// # Errors
///
/// - when writing to `output` fails
pub fn show(
    data: &BTreeMap<&str, Data>,
    output: &mut impl Write,
) -> io::Result<()> {
    show_bytes(data, output)?;
    writeln!(output)?;
    show_files(data, output)?;
    Ok(())
}

fn show_bytes(
    data: &BTreeMap<&str, Data>,
    output: &mut impl Write,
) -> io::Result<()> {
    writeln!(output, "# HELP stor_age_bytes_total Total size in bytes.")?;
    writeln!(output, "# TYPE stor_age_bytes_total gauge")?;

    for (dir, data) in data {
        writeln!(
            output,
            "stor_age_bytes_total{{dir=\"{}\"}} {}",
            dir,
            data.get_total_bytes()
        )?;
    }

    writeln!(output,)?;
    writeln!(
        output,
        "# HELP stor_age_bytes_accessed Accessed size in bytes."
    )?;
    writeln!(output, "# TYPE stor_age_bytes_accessed gauge")?;

    for (dir, data) in data {
        for age in data.get_ages() {
            writeln!(
                output,
                "stor_age_bytes_accessed{{dir=\"{}\",age=\"{}\"}} {}",
                dir,
                age,
                data.get_accessed_bytes(*age).unwrap()
            )?;
        }
    }

    writeln!(output,)?;
    writeln!(
        output,
        "# HELP stor_age_bytes_modified Modified size in bytes."
    )?;
    writeln!(output, "# TYPE stor_age_bytes_modified gauge")?;

    for (dir, data) in data {
        for age in data.get_ages() {
            writeln!(
                output,
                "stor_age_bytes_modified{{dir=\"{}\",age=\"{}\"}} {}",
                dir,
                age,
                data.get_modified_bytes(*age).unwrap()
            )?;
        }
    }

    Ok(())
}

fn show_files(
    data: &BTreeMap<&str, Data>,
    output: &mut impl Write,
) -> io::Result<()> {
    writeln!(output, "# HELP stor_age_files_total Total number of files.")?;
    writeln!(output, "# TYPE stor_age_files_total gauge")?;

    for (dir, data) in data {
        writeln!(
            output,
            "stor_age_files_total{{dir=\"{}\"}} {}",
            dir,
            data.get_total_files()
        )?;
    }

    writeln!(output,)?;
    writeln!(
        output,
        "# HELP stor_age_files_accessed Accessed number of files."
    )?;
    writeln!(output, "# TYPE stor_age_files_accessed gauge")?;

    for (dir, data) in data {
        for age in data.get_ages() {
            writeln!(
                output,
                "stor_age_files_accessed{{dir=\"{}\",age=\"{}\"}} {}",
                dir,
                age,
                data.get_accessed_files(*age).unwrap()
            )?;
        }
    }

    writeln!(output,)?;
    writeln!(
        output,
        "# HELP stor_age_files_modified Modified number of files."
    )?;
    writeln!(output, "# TYPE stor_age_files_modified gauge")?;

    for (dir, data) in data {
        for age in data.get_ages() {
            writeln!(
                output,
                "stor_age_files_modified{{dir=\"{}\",age=\"{}\"}} {}",
                dir,
                age,
                data.get_modified_files(*age).unwrap()
            )?;
        }
    }

    Ok(())
}
