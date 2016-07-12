use chrono::*;
use xdg;
use csv;

use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

#[derive(Clone, Copy)]
pub struct Entry {
    pub at: DateTime<UTC>,
    pub glucose: f32,
}

impl Entry {
    pub fn all() -> Vec<Entry> {
        let mut f = File::open(Entry::data_path()).unwrap();
        let mut data = String::new();
        f.read_to_string(&mut data).unwrap();

        let mut rdr = csv::Reader::from_string(data).has_headers(false);
        let mut list = Vec::new();
        for row in rdr.decode() {
            let (date, glucose): (String, f32) = row.unwrap();
            list.push(Entry {
                at: date.parse::<DateTime<UTC>>().unwrap(),
                glucose: glucose,
            });
        }
        list
    }

    fn data_path() -> PathBuf {
        xdg::BaseDirectories::with_prefix("gluj")
            .unwrap()
            .find_data_file("glucose.csv")
            .expect("Need file: ${XDG_DATA_HOME:-~/.local/share}/gluj/glucose.csv")
    }
}
