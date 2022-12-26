use std::collections::HashMap;

use bytesize::ByteSize;
use prettytable::{cell, format::FormatBuilder, Row, Table};
use smooth::Smooth;

use crate::Data;

pub fn show(data: &HashMap<&str, Data>) {
    let mut table = Table::new();
    let format = FormatBuilder::new().column_separator(' ').build();
    table.set_format(format);

    let mut titles = Row::empty();
    titles.add_cell(cell!(bu->"Directory"));
    titles.add_cell(cell!(bu->"Age"));
    titles.add_cell(cell!(bu->"Bytes"));
    titles.add_cell(cell!(bu->"Accessed"));
    titles.add_cell(cell!(bu->"Percent"));
    titles.add_cell(cell!(bu->"Modified"));
    titles.add_cell(cell!(bu->"Percent"));
    titles.add_cell(cell!(bu->"Files"));
    titles.add_cell(cell!(bu->"Accessed"));
    titles.add_cell(cell!(bu->"Percent"));
    titles.add_cell(cell!(bu->"Modified"));
    titles.add_cell(cell!(bu->"Percent"));
    table.set_titles(titles);

    for (dir, data) in data {
        let total_bytes = data.get_total_bytes();
        let total_files = data.get_total_files();

        let mut first = true;

        for age in data.get_ages() {
            let mut row = Row::empty();

            if first {
                row.add_cell(cell!(dir));
            } else {
                row.add_cell(cell!(""));
            }

            row.add_cell(cell!(r->age));

            if first {
                row.add_cell(
                    cell!(r->ByteSize(total_bytes).to_string_as(true)),
                );
            } else {
                row.add_cell(cell!(r->""));
            }

            let accessed_bytes = data.get_accessed_bytes(*age).unwrap();
            let modified_bytes = data.get_modified_bytes(*age).unwrap();

            let (accessed_bytes_percentage, modified_bytes_percentage) =
                percentage(total_bytes, accessed_bytes, modified_bytes);

            let accessed_bytes = ByteSize(accessed_bytes).to_string_as(true);
            let modified_bytes = ByteSize(modified_bytes).to_string_as(true);

            row.add_cell(cell!(r->accessed_bytes));
            row.add_cell(cell!(r->format!("{accessed_bytes_percentage}%")));

            row.add_cell(cell!(r->modified_bytes));
            row.add_cell(cell!(r->format!("{modified_bytes_percentage}%")));

            if first {
                row.add_cell(cell!(r->format!("{total_files}")));
            } else {
                row.add_cell(cell!(r->""));
            }

            let accessed_files = data.get_accessed_files(*age).unwrap();
            let modified_files = data.get_modified_files(*age).unwrap();

            let (accessed_files_percentage, modified_files_percentage) =
                percentage(total_files, accessed_files, modified_files);

            row.add_cell(cell!(r->accessed_files));
            row.add_cell(cell!(r->format!("{accessed_files_percentage}%")));

            row.add_cell(cell!(r->modified_files));
            row.add_cell(cell!(r->format!("{modified_files_percentage}%")));

            table.add_row(row);

            first = false;
        }
    }

    println!();
    table.printstd();
    println!();
}

#[allow(clippy::cast_precision_loss)]
fn percentage(total: u64, accessed: u64, modified: u64) -> (f64, f64) {
    if total == 0 {
        (0.0, 0.0)
    } else {
        let accessed_percentage =
            ((accessed as f64) / (total as f64) * 100.0).round_to(2);

        let modified_percentage =
            ((modified as f64) / (total as f64) * 100.0).round_to(2);

        (accessed_percentage, modified_percentage)
    }
}
