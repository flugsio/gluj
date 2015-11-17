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
    to: DateTime<UTC>,
    entries: HashMap<DateTime<UTC>, Vec<Entry>>,
}

#[derive(Clone, Copy)]
enum Show {
    Empty,
    Timeline,
    Glucose(i64),
}

impl fmt::Display for Show {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Show::Empty      => write!(f, "{}", "-"),
            &Show::Timeline   => write!(f, "{}", "|"),
            &Show::Glucose(x) if x < 0 || x > 33
                              => write!(f, "?"),
            &Show::Glucose(x) => {
                // cleaner version sadly only handles 0-15: write!(f, "{:X}", x)
                let y = char::from_digit(x as u32, 36).unwrap().to_uppercase().next().unwrap();
                write!(f, "{}", y)
            },
        }
    }
}

impl View {
    pub fn new(to: DateTime<UTC>, entries: Vec<Entry>) -> View {
        let mut grouped_entries = HashMap::new();
        for entry in entries {
            let rounded = floor_time(entry.at);
            let list = grouped_entries.entry(rounded).or_insert(Vec::new());
            list.push(entry);
        }
        View {
            to: floor_time(to),
            entries: grouped_entries,
        }
    }

    // TODO: wtf is this mess
    pub fn render(&self) -> String {
        let mut source = [Show::Empty; 32];
        let mut buffer = String::new();
        let mut now = self.to;
        for i in 0..source.len() {
            if now.minute() == 0 && now.hour() % 2 == 0 {
                source[i] = Show::Timeline;
            }
            match self.entries.get(&now) {
                Some(entry) => source[i] = Show::Glucose(entry.last().unwrap().glucose),
                _ => ()
            }
            buffer = format!("{}{}", source[i], buffer);
            now = now - Duration::minutes(15);
        }
        return buffer;
    }

}

/// Floor datetime down to nearest 15 minute block
fn floor_time(dt: DateTime<UTC>) -> DateTime<UTC> {
    let minute = dt.minute() / 15 * 15;
    UTC.ymd(dt.year(), dt.month(), dt.day()).and_hms(dt.hour(), minute, 0)
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
        let even_hour      = UTC.ymd(2015, 1, 15).and_hms(12,  0, 0);
        let even_hour_15   = UTC.ymd(2015, 1, 15).and_hms(12, 15, 0);
        let even_hour_30   = UTC.ymd(2015, 1, 15).and_hms(12, 30, 0);
        let uneven_hour    = UTC.ymd(2015, 1, 15).and_hms(13,  0, 0);
        let uneven_hour_30 = UTC.ymd(2015, 1, 15).and_hms(13, 30, 0);
        assert_eq!("-------|-------|-------|-------|".to_string(), View::new(even_hour, entries.clone()).render());
        assert_eq!("------|-------|-------|-------|-".to_string(), View::new(even_hour_15, entries.clone()).render());
        assert_eq!("-----|-------|-------|-------|--".to_string(), View::new(even_hour_30, entries.clone()).render());
        assert_eq!("---|-------|-------|-------|----".to_string(), View::new(uneven_hour, entries.clone()).render());
        assert_eq!("-|-------|-------|-------|------".to_string(), View::new(uneven_hour_30, entries.clone()).render());
    }

    #[test]
    pub fn test_entry() {
        let entry_date = UTC.ymd(2015, 1, 15).and_hms(11, 5, 30);
        let show_date  = UTC.ymd(2015, 1, 15).and_hms(12, 0,  0);
        let entries = vec!(Entry { at: entry_date, glucose: 7 } );
        assert_eq!("-------|-------|-------|---7---|".to_string(), View::new(show_date, entries).render());
    }
}
