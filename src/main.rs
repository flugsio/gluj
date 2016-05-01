extern crate chrono;
extern crate csv;
extern crate xdg;

use std::io::prelude::*;
use std::fs::File;
use chrono::*;

mod graph;

#[allow(dead_code)]
fn main() {
    let now = UTC::now();
    match std::env::args().nth(1) {
        Some(ref x) if x == "-h" => {
            println!("gluj     Show last 8 hours");
            println!("gluj -m  Show last 30 days");
            println!("gluj -h  Display this help");
        },
        Some(ref x) if x == "-m" => {
            let graph = graph::View::new(load_entries());
            println!("           0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16  17  18  19  20  21  22  23");
            for i in 0..30 {
                let day = now - Duration::days(29-i);
                println!("{} {}", day.format("%Y-%m-%d"), graph.render_day(day));
            }
        },
        None =>  {
            let graph = graph::View::new(load_entries());
            println!("{}", graph.render(now));
        },
        Some(_) => {}
    }
}

fn load_entries() -> Vec<graph::Entry> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("gluj").unwrap();
    let path = xdg_dirs.find_data_file("glucose.csv")
        .expect("Need file: $XDG_DATA_HOME/gluj/glucose.csv");

    let mut f = File::open(path).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let mut rdr = csv::Reader::from_string(data).has_headers(false);
    let mut list = Vec::new();
    for row in rdr.decode() {
        let (date, glucose): (String, f32) = row.unwrap();
        list.push(graph::Entry { at: date.parse::<DateTime<UTC>>().unwrap(), glucose: glucose });
    }
    list
}
