use chrono::*;
use xdg;
use csv;

use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

#[derive(Clone, Copy)]
pub struct Entry {
    pub at: DateTime<Utc>,
    pub glucose: f32,
}

impl Entry {
    pub fn parse(at: DateTime<Utc>, glucose: &str) -> Entry {
        Entry {
            at: at,
            glucose: glucose.parse().unwrap(),
        }
    }

    pub fn store(&self) {
        let entry = (
            self.at.with_timezone(&Local).format("%Y-%m-%dT%H:%M:%S%z").to_string(),
            format!(" {:.1}", self.glucose)
        );
        let f = OpenOptions::new()
            .append(true).open(Entry::data_path("glucose.csv"))
            .unwrap();
        let mut wtr = csv::Writer::from_writer(f);
        wtr.serialize(entry).unwrap();
        wtr.flush().unwrap(); // flush is needed to ensure full write
    }

    pub fn all() -> Vec<Entry> {
        let mut data = Entry::read("glucose.csv");
        let mut list = Vec::new();
        for row in data.deserialize() {
            let (date, glucose): (String, f32) = row.unwrap();
            list.push(Entry {
                at: date.parse().unwrap(),
                glucose: glucose,
            });
        }
        list
    }

    fn read(name: &str) -> csv::Reader<std::fs::File> {
        csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_reader(File::open(Entry::data_path(name)).unwrap())
    }

    fn data_path(name: &str) -> PathBuf {
        xdg::BaseDirectories::with_prefix("gluj")
            .unwrap()
            .find_data_file(name)
            .expect("Need file: ${XDG_DATA_HOME:-~/.local/share}/gluj/glucose.csv")
    }
}
