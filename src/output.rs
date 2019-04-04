use acc::Acc;
use bytesize::ByteSize;

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Output {
        Pretty,
        Oneline,
    }
}

pub fn pretty(dir: &str, acc: Acc, age: &u64) {
    let Acc { total, access, modify } = acc;

    let (a_p, m_p) = if total == 0 {
        (0.0, 0.0)
    } else {
        let a_p = ((access as f64) / (total as f64) * 100.0).round();
        let m_p = ((modify as f64) / (total as f64) * 100.0).round();
        (a_p, m_p)
    };

    let t_b = ByteSize(total).to_string_as(true);
    let a_b = ByteSize(access).to_string_as(true);
    let m_b = ByteSize(modify).to_string_as(true);

    println!("{}: {}", dir, t_b);

    println!(
        "unaccessed for {} days: {}% ({})",
        age,
        a_p,
        a_b,
    );

    println!(
        "unmodified for {} days: {}% ({})",
        age,
        m_p,
        m_b,
    );
}

pub fn oneline(dir: &str, acc: Acc) {
    let Acc { total, access, modify } = acc;

    println!(
        "{}:{}:{}:{}",
        total,
        access,
        modify,
        dir,
    );
}
