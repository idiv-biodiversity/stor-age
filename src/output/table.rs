use std::collections::HashMap;

use bytesize::ByteSize;
use prettytable::{cell, format::FormatBuilder, Row, Table};

use crate::Data;

pub fn show(data: HashMap<&str, Data>) {
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

    for (dir, data) in &data {
        let t_b = data.get_total_bytes();
        let t_f = data.get_total_files();

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
                row.add_cell(cell!(r->ByteSize(t_b).to_string_as(true)));
            } else {
                row.add_cell(cell!(r->""));
            }

            let a_b = data.get_accessed_bytes(*age).unwrap();
            let m_b = data.get_modified_bytes(*age).unwrap();

            let (a_b_p, m_b_p) = if t_b == 0 {
                (0.0, 0.0)
            } else {
                let a_p =
                    ((a_b as f64) / (t_b as f64) * 10000.0).round() / 100.0;
                let m_p =
                    ((m_b as f64) / (t_b as f64) * 10000.0).round() / 100.0;
                (a_p, m_p)
            };

            let a_b = ByteSize(a_b).to_string_as(true);
            let m_b = ByteSize(m_b).to_string_as(true);

            row.add_cell(cell!(r->a_b));
            row.add_cell(cell!(r->format!("{}%", a_b_p)));

            row.add_cell(cell!(r->m_b));
            row.add_cell(cell!(r->format!("{}%", m_b_p)));

            if first {
                row.add_cell(cell!(r->format!("{}", t_f)));
            } else {
                row.add_cell(cell!(r->""));
            }

            let a_f = data.get_accessed_files(*age).unwrap();
            let m_f = data.get_modified_files(*age).unwrap();

            let (a_f_p, m_f_p) = if t_f == 0 {
                (0.0, 0.0)
            } else {
                let a_p =
                    ((a_f as f64) / (t_f as f64) * 10000.0).round() / 100.0;
                let m_p =
                    ((m_f as f64) / (t_f as f64) * 10000.0).round() / 100.0;
                (a_p, m_p)
            };

            row.add_cell(cell!(r->a_f));
            row.add_cell(cell!(r->format!("{}%", a_f_p)));

            row.add_cell(cell!(r->m_f));
            row.add_cell(cell!(r->format!("{}%", m_f_p)));

            table.add_row(row);

            first = false;
        }
    }

    println!();
    table.printstd();
    println!();
}
