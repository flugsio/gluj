extern crate chrono;
extern crate csv;
extern crate xdg;

use chrono::*;

mod graph;
mod entry;

#[allow(dead_code)]
fn main() {
    match std::env::args().nth(1) {
        Some(ref x) if x == "-h" => {
            println!("gluj     Show last 8 hours");
            println!("gluj -m  Show last 30 days");
            println!("gluj -h  Display this help");
        },
        Some(ref x) if x == "-m" => render_month(),
        Some(ref x) => add(x),
        None => render_recent(),
    }
}

fn add(glucose: &str) {
    let now = UTC::now();
    entry::Entry::parse(now, glucose).store();
}

fn render_month() {
    let now = UTC::now();
    let graph = graph::View::new(entry::Entry::all());
    println!("           0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16  17  18  19  20  21  22  23");
    for i in 0..30 {
        let day = now - Duration::days(29-i);
        println!("{} {}", day.format("%Y-%m-%d"), graph.render_day(day));
    }
}

fn render_recent() {
    let now = UTC::now();
    let graph = graph::View::new(entry::Entry::all());
    println!("{}", graph.render_recent(now));
}
