use core::fmt;
use std::io;
pub(crate) mod files_n_folders;
pub(crate) mod globals;

use chrono::{Datelike, NaiveDate};

use crate::utils::globals::MONTHS;

// @todo
// - Dateiproduktion verbessern

// Sometimes you just want
// "January 2026" as Date,
//
// This is stored internally as
// January 1, 2026 with 'month_only' == true
#[derive(PartialEq, Debug, Clone)]
pub struct FlexibleDate {
    pub(crate) datum: Option<NaiveDate>,
    pub(crate) month_only: bool,
    pub(crate) is_parsed: bool,
    pub(crate) input: String,
}

impl fmt::Display for FlexibleDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_parsed {
            match self.month_only {
                true => {
                    let dt = self.datum.unwrap();
                    let monat = MONTHS[dt.month0() as usize];
                    write!(f, "{} {}", monat, dt.year())
                }
                _ => write!(f, "{}", self.datum.unwrap().format("%d.%m.%Y")),
            }
        } else {
            if self.input.is_empty() {
                return write!(f, "--");
            }
            write!(f, "{}", self.input)
        }
    }
}

impl FlexibleDate {
    pub fn new_empty() -> FlexibleDate {
        FlexibleDate {
            datum: None,
            month_only: false,
            is_parsed: false,
            input: "--".to_string(),
        }
    }
}

/// Minimal way to ask the user for input
/// on a terminal
pub(crate) fn get_user_input(question: &str, default: &str) -> String {
    println!("suv -> {}", question);

    if !default.is_empty() {
        println!("(Empty for `{}`)", &default);
    }

    let mut line = String::from(" ");

    io::stdin()
        .read_line(&mut line)
        .expect("Something went wrong trying to read your input"); // @todo

    if line.trim().is_empty() {
        default.to_string()
    } else {
        line.trim().to_string()
    }
}

// Möchte der Nutzer ein Datum erfassen?
pub(crate) fn get_optional_user_date(
    question: &str,
    default: &Option<FlexibleDate>,
) -> FlexibleDate {
    let mut prompt = format!("suv -> {}", question);

    // return None;
    // @todo
    if let Some(def) = default {
        prompt = format!(
            "{} [{} oder `--` für *kein* Datum])",
            &prompt,
            &def.to_string()
        );
    }

    let mut line = String::from("");
    println!("{}", prompt);

    io::stdin()
        .read_line(&mut line)
        .expect("Something went wrong trying to read your input"); // @todo

    if line.trim().is_empty() {
        println!("LINE is empty, default is {:?}", default);
        if default.is_some() {
            println!("Ok: {}", default.as_ref().unwrap());
            return default.as_ref().unwrap().to_owned();
        }
        return FlexibleDate::new_empty();
    }

    if &line.to_ascii_lowercase() == "--" {
        return FlexibleDate::new_empty();
    }
    FlexibleDate::from_str_future(&line)
}

pub(crate) fn get_yes_no_user_input(question: &str, default: &bool) -> bool {
    println!("suv -> {}", question);

    let mut line = String::from(" ");

    io::stdin()
        .read_line(&mut line)
        .expect("Something went wrong trying to read your input"); // @todo

    if line.trim().is_empty() {
        *default
    } else {
        line.parse().unwrap_or_default() // <-- @todo
    }
}
pub(crate) fn ask_option(question: &str, options: &[String]) -> String {
    for (index, option) in options.iter().enumerate() {
        println!("{index} -- {option}");
    }
    println!();
    let s = get_user_input(question, "").trim().to_lowercase();
    match s.parse::<usize>() {
        Ok(u) => match options.get(u) {
            Some(response) => response.to_string(),
            None => panic!("Not understood, @todo needs programming"),
        },
        Err(e) => panic!("@todo, needs programming {e}"),
    }
}
