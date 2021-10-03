use termion::{color, clear, cursor};

fn main() {
    const rows: [&str; 6] = [
        "▒ ▒▒▒▒ ▒▒ ▒ ▒ ▒▒▒ ▒▒▒▒ ▒ ▒",
        "▒▒ ▒    ▒        ▒ ▒   ▒ ▒",
        "▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒  ▒ ▒ ",
        "▒▒▒▒▒▒▒▒▒▒▒▒              ",
        "▒▒                     ▒▒▒",
        "▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒",
    ];

    print!("{}{}{}Simulation:\n", clear::All, cursor::Goto(1,1), color::Fg(color::Red));
    write_rows(rows, 2);
    print!("\n")
}

fn write_rows(rows: [&str; 6], offset: usize) {
    for item in rows.iter().enumerate() {
        let (i, row): (usize, &&str) = item;
        write_row(*row, offset + i);
    }
}

fn write_row(content: &str, row: usize) {
    print!("{}{}", cursor::Goto(1, row as u16), content);
}
