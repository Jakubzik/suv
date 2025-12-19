use chrono::{Datelike, NaiveDate, Utc};

use crate::utils::FlexibleDate;
use crate::utils::globals::{DAYS, MONATE, MONTHS, TAGE, TIME_FOR_BA_PAPER_PRODUCTION};

pub(crate) fn get_default_anmelde_from_abgabe(abgabe: FlexibleDate) -> FlexibleDate {
    if abgabe.is_parsed {
        let anmelde = abgabe
            .datum
            .unwrap()
            .checked_sub_days(TIME_FOR_BA_PAPER_PRODUCTION);
        return FlexibleDate {
            datum: anmelde,
            month_only: abgabe.month_only,
            is_parsed: true,
            input: format!(
                "calculated from abgabe {}",
                abgabe.datum.unwrap().format("%d.%m.%YYYY")
            ),
        };
    }
    FlexibleDate::new_empty()
}

// Januar = 1, Februar = 2 etc.
//
// Versucht, den String `s_in` als Monat zu
// interpretieren. Interpretierbare Werte sind
// January, Januar, Janu, Jan
pub fn read_as_month(s_in: &str) -> Option<usize> {
    if s_in.trim().len() < 3 {
        return None;
    }
    // Wir vergleichen die ersten drei Zeichen
    // als lowercase
    let potential_month = s_in[0..3].to_lowercase();

    // Erst Englisch...
    let m_engl = MONTHS
        .iter()
        .position(|m| m.to_lowercase()[0..3] == potential_month)
        .map(|month| month + 1);

    // falls englisch nicht passt, versuchen wir deutsch
    if m_engl.is_none() {
        MONATE
            .iter()
            .position(|m| m.to_lowercase()[0..3] == potential_month)
            .map(|mon| mon + 1)
    } else {
        m_engl
    }
}

// Montag = 1, Dienstag = 2 ...
//
// Versucht, den String `s_in` als Wochentag zu
// interpretieren. Interpretierbare Werte sind
// Montag, Monday, Mont, Mond etc.
pub fn read_as_day(s_in: &str) -> Option<usize> {
    if s_in.trim().len() < 3 {
        return None;
    }
    let potential_day = s_in.trim().to_lowercase()[0..3].to_string();
    let d_englisch = DAYS
        .iter()
        .position(|m| m == &potential_day)
        .map(|day| day + 1);

    if d_englisch.is_none() {
        TAGE.iter()
            .position(|m| m == &potential_day)
            .map(|day| day + 1)
    } else {
        d_englisch
    }
}

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
