use chrono::{Datelike, NaiveDate, Utc};

const MONTHS: &'static [&'static str] = &[
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];
const DAYS: &'static [&'static str] = &["mon", "tue", "wed", "thu", "fri", "sat", "sun"];
const MONATE: &'static [&'static str] = &[
    "jan", "feb", "mär", "apr", "mai", "jun", "jul", "aug", "sep", "okt", "nov", "dez",
];

const TAGE: &'static [&'static str] = &["mon", "die", "mit", "don", "fri", "sam", "son"];

// Gibt das aktuelle Jahr zurück, oder
// ein Jahr +- i_dist.
// get_year(0) -> aktuelles Jahr
// get_year(-1) -> letztes Jahr
// get_year(1) -> nächstes Jahr
pub fn get_year(i_dist: i32) -> i32 {
    Utc::now().naive_local().year() + i_dist
}

// Utility
// January = 1, February = 2 etc.
pub fn get_current_month() -> u32 {
    Utc::now().naive_local().month()
}

pub fn today() -> NaiveDate {
    Utc::now().naive_local().date()
}

// Utility
// January = 1, February = 2 etc.
pub fn get_current_weekday() -> u32 {
    Utc::now().naive_local().weekday().num_days_from_sunday()
}

// Januar = 1, Februar = 2 etc.
//
// Versucht, den String `s_in` als Monat zu
// interpretieren. Interpretierbare Werte sind
// January, Januar, Janu, Jan
pub fn read_as_month(s_in: &str) -> Option<usize> {
    let words: Vec<&str> = s_in.split_whitespace().collect();

    let potential_month = words[0].trim().to_lowercase()[0..3].to_string();
    if let Some(month) = MONTHS.iter().position(|m| m == &potential_month) {
        return Some(month + 1);
    } else if let Some(month) = MONATE.iter().position(|m| m == &potential_month) {
        return Some(month + 1);
    } else {
        None
    }
}

// Montag = 1, Dienstag = 2 ...
//
// Versucht, den String `s_in` als Wochentag zu
// interpretieren. Interpretierbare Werte sind
// Montag, Monday, Mont, Mond etc.
pub fn read_as_day(s_in: &str) -> Option<usize> {
    let potential_day = s_in.trim().to_lowercase()[0..3].to_string();
    if let Some(day) = DAYS.iter().position(|m| m == &potential_day) {
        return Some(day + 1);
    } else if let Some(day) = TAGE.iter().position(|m| m == &potential_day) {
        return Some(day + 1);
    } else {
        None
    }
}
