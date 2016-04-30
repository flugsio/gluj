extern crate chrono;
extern crate csv;

use std::io::prelude::*;
use std::fs::File;
use chrono::*;

mod graph;

#[allow(dead_code)]
fn main() {
    let entries = load_entries("/home/flugsio/.glucose");
    let now = UTC::now();
    let graph = graph::View::new(entries);
    println!("{}", graph.render(now));
}

fn load_entries(path: &str) -> Vec<graph::Entry> {
    let mut f = File::open(path).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let mut rdr = csv::Reader::from_string(data).has_headers(false);
    let mut list = Vec::new();
    for row in rdr.decode() {
        let (date, glucose): (String, i64) = row.unwrap();
        list.push(graph::Entry { at: date.parse::<DateTime<UTC>>().unwrap(), glucose: glucose });
    }
    list
}
