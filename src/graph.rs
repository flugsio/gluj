use chrono::*;
use std::fmt;
use std::char;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Entry {
    pub at: DateTime<UTC>,
    pub glucose: f32,
}

pub struct View {
    latest: Option<DateTime<UTC>>,
    entries: HashMap<DateTime<UTC>, Vec<Entry>>,
}

#[derive(Clone, Copy)]
enum Grapheme {
    Empty,
    Timeline,
    Waitline,
    Glucose(f32),
}

impl fmt::Display for Grapheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Grapheme::Empty      => write!(f, " "),
            &Grapheme::Timeline   => write!(f, "|"),
            &Grapheme::Waitline   => write!(f, "-"),
            &Grapheme::Glucose(x) if -1.0 < x && x < 34.0 => {
                let y = char::from_digit(x.round() as u32, 36).unwrap().to_uppercase().next().unwrap();
                write!(f, "{}", y)
            },
            &Grapheme::Glucose(_) => write!(f, "?"),
        }
    }
}

impl View {
    pub fn new(entries: Vec<Entry>) -> View {
        let latest = entries.last().map(|e| e.at.clone());
        let mut grouped_entries = HashMap::new();
        for entry in entries {
            grouped_entries
                .entry(floor_time(entry.at))
                .or_insert(Vec::new())
                .push(entry);
        }
        View {
            latest: latest,
            entries: grouped_entries,
        }
    }

    pub fn render(&self, to: DateTime<UTC>) -> String {
        let mut buffer = String::new();
        let graphemes = (0..32).map(|i|
            self.grapheme_at(floor_time(to) - Duration::minutes(i * 15))
        );
        for entry in graphemes {
            buffer = format!("{}{}", entry, buffer);
        }
        return buffer;
    }

    fn grapheme_at(&self, time: DateTime<UTC>) -> Grapheme {
        match self.entries.get(&time) {
            Some(entry) => Grapheme::Glucose(entry.last().unwrap().glucose),
            None if even_full_hour(time) => Grapheme::Timeline,
            None => match self.latest {
                Some(latest) if (time < latest) => Grapheme::Empty,
                _ => Grapheme::Waitline,
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
    use super::Grapheme;
    use super::View;

    #[test]
    pub fn test_glucose_letter_representation() {
        // implementation needs to cover 0-33
        assert_eq!("0", Grapheme::Glucose(0.0).to_string());
        assert_eq!("1", Grapheme::Glucose(1.0).to_string());
        assert_eq!("2", Grapheme::Glucose(2.0).to_string());
        assert_eq!("3", Grapheme::Glucose(3.0).to_string());
        assert_eq!("4", Grapheme::Glucose(4.0).to_string());
        assert_eq!("5", Grapheme::Glucose(5.0).to_string());
        assert_eq!("6", Grapheme::Glucose(6.0).to_string());
        assert_eq!("7", Grapheme::Glucose(7.0).to_string());
        assert_eq!("8", Grapheme::Glucose(8.0).to_string());
        assert_eq!("9", Grapheme::Glucose(9.0).to_string());
        assert_eq!("A", Grapheme::Glucose(10.0).to_string());
        assert_eq!("B", Grapheme::Glucose(11.0).to_string());
        assert_eq!("C", Grapheme::Glucose(12.0).to_string());
        assert_eq!("D", Grapheme::Glucose(13.0).to_string());
        assert_eq!("E", Grapheme::Glucose(14.0).to_string());
        assert_eq!("F", Grapheme::Glucose(15.0).to_string());
        assert_eq!("G", Grapheme::Glucose(16.0).to_string());

        assert_eq!("U", Grapheme::Glucose(30.0).to_string());
        assert_eq!("X", Grapheme::Glucose(33.0).to_string());

        // test out of range
        assert_eq!("?", Grapheme::Glucose(-1.0).to_string());
        assert_eq!("?", Grapheme::Glucose(34.0).to_string());
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
        let entries = vec!(Entry { at: entry_date, glucose: 7.0 } );
        assert_eq!("       |       |       |   7---|".to_string(), View::new(entries).render(show_date));
    }
}
