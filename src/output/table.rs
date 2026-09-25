use std::collections::HashMap;
use std::hash::BuildHasher;

use bytesize::ByteSize;
use comfy_table::presets;
use comfy_table::{Attribute, Cell, CellAlignment, Row, Table};
use smooth::Smooth;

use crate::Data;

pub fn show<S: BuildHasher>(data: &HashMap<&str, Data, S>, markdown: bool) {
    let mut table = Table::new();

    if markdown {
        table.load_style(presets::ASCII_MARKDOWN);
    } else {
        table.load_style(presets::NOTHING);
    }

    let header_attributes = if markdown {
        vec![]
    } else {
        vec![Attribute::Bold, Attribute::Underlined]
    };

    table.set_header(vec![
        Cell::new("Directory").add_attributes(header_attributes.clone()),
        Cell::new("Age").add_attributes(header_attributes.clone()),
        Cell::new("Bytes").add_attributes(header_attributes.clone()),
        Cell::new("Accessed").add_attributes(header_attributes.clone()),
        Cell::new("Percent").add_attributes(header_attributes.clone()),
        Cell::new("Modified").add_attributes(header_attributes.clone()),
        Cell::new("Percent").add_attributes(header_attributes.clone()),
        Cell::new("Files").add_attributes(header_attributes.clone()),
        Cell::new("Accessed").add_attributes(header_attributes.clone()),
        Cell::new("Percent").add_attributes(header_attributes.clone()),
        Cell::new("Modified").add_attributes(header_attributes.clone()),
        Cell::new("Percent").add_attributes(header_attributes),
    ]);

    for (dir, data) in data {
        let total_bytes = data.get_total_bytes();
        let total_files = data.get_total_files();

        let mut first = true;

        for age in data.get_ages() {
            let row = row(dir, *age, data, total_bytes, total_files, first);
            table.add_row(row);
            first = false;
        }
    }

    println!();
    println!("{table}");
    println!();
}

fn row(
    dir: &str,
    age: u64,
    data: &Data,
    total_bytes: u64,
    total_files: u64,
    first: bool,
) -> Row {
    let mut row = Row::new();

    if first {
        row.add_cell(Cell::new(dir));
    } else {
        row.add_cell(Cell::new(""));
    }

    row.add_cell(Cell::new(age).set_alignment(CellAlignment::Right));

    if first {
        row.add_cell(
            Cell::new(ByteSize(total_bytes).display().iec())
                .set_alignment(CellAlignment::Right),
        );
    } else {
        row.add_cell(Cell::new("").set_alignment(CellAlignment::Right));
    }

    let accessed_bytes = data.get_accessed_bytes(age).unwrap_or_default();
    let modified_bytes = data.get_modified_bytes(age).unwrap_or_default();

    let (accessed_bytes_percentage, modified_bytes_percentage) =
        percentage(total_bytes, accessed_bytes, modified_bytes);

    let accessed_bytes = ByteSize(accessed_bytes).display().iec();
    let modified_bytes = ByteSize(modified_bytes).display().iec();

    row.add_cell(
        Cell::new(accessed_bytes).set_alignment(CellAlignment::Right),
    );
    row.add_cell(
        Cell::new_owned(format!("{accessed_bytes_percentage}%"))
            .set_alignment(CellAlignment::Right),
    );

    row.add_cell(
        Cell::new(modified_bytes).set_alignment(CellAlignment::Right),
    );
    row.add_cell(
        Cell::new_owned(format!("{modified_bytes_percentage}%"))
            .set_alignment(CellAlignment::Right),
    );

    if first {
        row.add_cell(
            Cell::new_owned(format!("{total_files}"))
                .set_alignment(CellAlignment::Right),
        );
    } else {
        row.add_cell(Cell::new("").set_alignment(CellAlignment::Right));
    }

    let accessed_files = data.get_accessed_files(age).unwrap_or_default();
    let modified_files = data.get_modified_files(age).unwrap_or_default();

    let (accessed_files_percentage, modified_files_percentage) =
        percentage(total_files, accessed_files, modified_files);

    row.add_cell(
        Cell::new(accessed_files).set_alignment(CellAlignment::Right),
    );
    row.add_cell(
        Cell::new_owned(format!("{accessed_files_percentage}%"))
            .set_alignment(CellAlignment::Right),
    );

    row.add_cell(
        Cell::new(modified_files).set_alignment(CellAlignment::Right),
    );
    row.add_cell(
        Cell::new_owned(format!("{modified_files_percentage}%"))
            .set_alignment(CellAlignment::Right),
    );

    row
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
