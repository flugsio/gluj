use chrono::*;
use std::fmt;
use std::char;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Entry {
    pub at: DateTime<UTC>,
    pub glucose: i64,
}

pub struct View {
    latest: Option<DateTime<UTC>>,
    entries: HashMap<DateTime<UTC>, Vec<Entry>>,
}

#[derive(Clone, Copy)]
enum Show {
    Empty,
    Timeline,
    Waitline,
    Glucose(i64),
}

impl fmt::Display for Show {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Show::Empty      => write!(f, " "),
            &Show::Timeline   => write!(f, "|"),
            &Show::Waitline   => write!(f, "-"),
            &Show::Glucose(x) if -1 < x && x < 34 => {
                let y = char::from_digit(x as u32, 36).unwrap().to_uppercase().next().unwrap();
                write!(f, "{}", y)
            },
            &Show::Glucose(_) => write!(f, "?"),
        }
    }
}

impl View {
    pub fn new(entries: Vec<Entry>) -> View {
        let mut grouped_entries = HashMap::new();
        let latest = entries.last().map(|e| e.at.clone());
        for entry in entries {
            let rounded = floor_time(entry.at);
            let list = grouped_entries.entry(rounded).or_insert(Vec::new());
            list.push(entry);
        }
        View {
            latest: latest,
            entries: grouped_entries,
        }
    }

    pub fn render(&self, to: DateTime<UTC>) -> String {
        let mut buffer = String::new();
        let entries = (0..32).map(|i|
            self.entry_for(floor_time(to) - Duration::minutes(i * 15))
        );
        for entry in entries {
            buffer = format!("{}{}", entry, buffer);
        }
        return buffer;
    }

    fn entry_for(&self, time: DateTime<UTC>) -> Show {
        match self.entries.get(&time) {
            Some(entry) => Show::Glucose(entry.last().unwrap().glucose),
            None if even_full_hour(time) => Show::Timeline,
            None => match self.latest {
                Some(latest) if (time < latest) => Show::Empty,
                _ => Show::Waitline,
            },
        }
    }

}

/// Floor datetime down to nearest 15 minute block
fn floor_time(dt: DateTime<UTC>) -> DateTime<UTC> {
    let minute = dt.minute() / 15 * 15;
    UTC.ymd(dt.year(), dt.month(), dt.day()).and_hms(dt.hour(), minute, 0)
}

fn even_full_hour(time: DateTime<UTC>) -> bool {
    time.minute() == 0 && time.hour() % 2 == 0
}


#[cfg(test)]
mod tests {
    use chrono::*;
    use super::Entry;
    use super::Show;
    use super::View;

    #[test]
    pub fn test_glucose_letter_representation() {
        // implementation needs to cover 0-33
        assert_eq!("0", Show::Glucose(0).to_string());
        assert_eq!("1", Show::Glucose(1).to_string());
        assert_eq!("2", Show::Glucose(2).to_string());
        assert_eq!("3", Show::Glucose(3).to_string());
        assert_eq!("4", Show::Glucose(4).to_string());
        assert_eq!("5", Show::Glucose(5).to_string());
        assert_eq!("6", Show::Glucose(6).to_string());
        assert_eq!("7", Show::Glucose(7).to_string());
        assert_eq!("8", Show::Glucose(8).to_string());
        assert_eq!("9", Show::Glucose(9).to_string());
        assert_eq!("A", Show::Glucose(10).to_string());
        assert_eq!("B", Show::Glucose(11).to_string());
        assert_eq!("C", Show::Glucose(12).to_string());
        assert_eq!("D", Show::Glucose(13).to_string());
        assert_eq!("E", Show::Glucose(14).to_string());
        assert_eq!("F", Show::Glucose(15).to_string());
        assert_eq!("G", Show::Glucose(16).to_string());

        assert_eq!("U", Show::Glucose(30).to_string());
        assert_eq!("X", Show::Glucose(33).to_string());

        // test out of range
        assert_eq!("?", Show::Glucose(-1).to_string());
        assert_eq!("?", Show::Glucose(34).to_string());
    }

    #[test]
    pub fn test_date_works() {
        let dt1 = UTC.ymd(2015, 1, 15).and_hms(12, 0, 0);
        let dt2 = UTC.ymd(2015, 1, 15).and_hms(12, 0, 2);
        let dt3 = dt2.with_second(0).unwrap();
        assert!(1i64 == 1);
        assert_eq!(dt1, dt3);
    }

    #[test]
    pub fn test_render_eight_empty_hours() {
        let entries = vec!();
        let view = View::new(entries);
        let even_hour      = UTC.ymd(2015, 1, 15).and_hms(12,  0, 0);
        let even_hour_15   = UTC.ymd(2015, 1, 15).and_hms(12, 15, 0);
        let even_hour_30   = UTC.ymd(2015, 1, 15).and_hms(12, 30, 0);
        let uneven_hour    = UTC.ymd(2015, 1, 15).and_hms(13,  0, 0);
        let uneven_hour_30 = UTC.ymd(2015, 1, 15).and_hms(13, 30, 0);
        assert_eq!("-------|-------|-------|-------|".to_string(), view.render(even_hour));
        assert_eq!("------|-------|-------|-------|-".to_string(), view.render(even_hour_15));
        assert_eq!("-----|-------|-------|-------|--".to_string(), view.render(even_hour_30));
        assert_eq!("---|-------|-------|-------|----".to_string(), view.render(uneven_hour));
        assert_eq!("-|-------|-------|-------|------".to_string(), view.render(uneven_hour_30));
    }

    #[test]
    pub fn test_entry() {
        let entry_date = UTC.ymd(2015, 1, 15).and_hms(11, 5, 30);
        let show_date  = UTC.ymd(2015, 1, 15).and_hms(12, 0,  0);
        let entries = vec!(Entry { at: entry_date, glucose: 7 } );
        assert_eq!("       |       |       |   7---|".to_string(), View::new(entries).render(show_date));
    }
}
