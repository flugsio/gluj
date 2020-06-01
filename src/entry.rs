use chrono::*;
use xdg;
use csv;

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
        let mut data = Entry::read("all.csv");
        data.append(&mut Entry::read("new.csv"));

        data.iter().filter_map(|row| if let Ok((date, glucose)) = row {
            Some(Entry {
                at: date.parse().unwrap(),
                glucose: *glucose,
            })
        } else {
            None
        }).collect()
    }

    fn read(name: &str) -> Vec<Result<(String, f32), csv::Error>> {
        csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .has_headers(false)
            .from_reader(File::open(Entry::data_path(name)).unwrap())
            .deserialize()
            .collect()
    }

    fn data_path(name: &str) -> PathBuf {
        xdg::BaseDirectories::with_prefix("gluj")
            .unwrap()
            .find_data_file(name)
            .expect(&format!("Need file: ${{XDG_DATA_HOME:-~/.local/share}}/gluj/{}", name))
    }
}
