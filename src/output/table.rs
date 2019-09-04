use bytesize::ByteSize;
use prettytable::{cell, format::FormatBuilder, Row, Table};
use std::collections::HashMap;

use crate::Acc;

pub fn show(data: HashMap<&str, Acc>) {
    let mut table = Table::new();
    let format = FormatBuilder::new().column_separator(' ').build();
    table.set_format(format);

    let mut titles = Row::empty();
    titles.add_cell(cell!(bu->"Directory"));
    titles.add_cell(cell!(bu->"Total"));
    titles.add_cell(cell!(bu->"Age"));
    titles.add_cell(cell!(bu->"Accessed"));
    titles.add_cell(cell!(bu->"Percent"));
    titles.add_cell(cell!(bu->"Modified"));
    titles.add_cell(cell!(bu->"Percent"));
    table.set_titles(titles);

    for (dir, acc) in &data {
        let total = acc.get_total_bytes();
        let mut first = true;

        for age in acc.get_ages() {
            let mut row = Row::empty();

            if first {
                row.add_cell(cell!(dir));
                row.add_cell(cell!(r->ByteSize(total).to_string_as(true)));
                first = false;
            } else {
                row.add_cell(cell!(""));
                row.add_cell(cell!(r->""));
            }

            let accessed = acc.get_accessed_bytes(*age).unwrap();
            let modified = acc.get_modified_bytes(*age).unwrap();

            let (a_p, m_p) = if total == 0 {
                (0.0, 0.0)
            } else {
                let a_p = ((accessed as f64) / (total as f64) * 10000.0)
                    .round()
                    / 100.0;
                let m_p = ((modified as f64) / (total as f64) * 10000.0)
                    .round()
                    / 100.0;
                (a_p, m_p)
            };

            let a_b = ByteSize(accessed).to_string_as(true);
            let m_b = ByteSize(modified).to_string_as(true);

            row.add_cell(cell!(r->age));

            row.add_cell(cell!(r->a_b));
            row.add_cell(cell!(r->format!("{}%", a_p)));

            row.add_cell(cell!(r->m_b));
            row.add_cell(cell!(r->format!("{}%", m_p)));

            table.add_row(row);
        }
    }

    println!();
    table.printstd();
    println!();
}
