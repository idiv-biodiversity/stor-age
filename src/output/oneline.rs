use std::collections::BTreeMap;
use std::io::{self, Write};

use crate::Data;

/// # Errors
///
/// - when writing to `output` fails
pub fn show(
    dir: &str,
    data: &Data,
    output: &mut impl Write,
) -> io::Result<()> {
    let t_b = data.get_total_bytes();
    let t_f = data.get_total_files();

    for age in data.get_ages() {
        let a_b = data.get_accessed_bytes(*age).unwrap_or_default();
        let m_b = data.get_modified_bytes(*age).unwrap_or_default();
        let a_f = data.get_accessed_files(*age).unwrap_or_default();
        let m_f = data.get_modified_files(*age).unwrap_or_default();

        writeln!(output, "{age}:{t_b}:{a_b}:{m_b}:{t_f}:{a_f}:{m_f}:{dir}")?;
    }

    Ok(())
}

/// # Errors
///
/// - when writing to `output` fails
pub fn show_all(
    data: &BTreeMap<&str, Data>,
    output: &mut impl Write,
) -> io::Result<()> {
    for (dir, data) in data {
        show(dir, data, output)?;
    }

    Ok(())
}
