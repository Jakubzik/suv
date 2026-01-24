use core::fmt;
use std::io;
pub(crate) mod error;
pub(crate) mod files_n_folders;
pub(crate) mod globals;

use chrono::{Datelike, NaiveDate};

use crate::{
    commands::Cmd,
    utils::{
        error::{Kind, SuvError},
        globals::{COMMANDS, MONTHS},
    },
};

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

pub(crate) struct InputCheck {
    pub(crate) must_not_be_empty: bool,
    pub(crate) default_value: String,
    pub(crate) check_format: Option<fn(&str) -> bool>,
}

/// Minimal way to ask the user for input
/// on a terminal
// pub(crate) fn get_user_input(question: &str, default: &str) -> String {
pub(crate) fn get_user_input(question: &str, check: &InputCheck) -> Result<String, SuvError> {
    println!("suv -> {}", question);

    if !check.default_value.is_empty() {
        println!("(Empty for `{}`)", &check.default_value);
    }

    let mut line = String::from(" ");

    io::stdin()
        .read_line(&mut line)
        .expect("Something went wrong trying to read your input"); // @todo

    if line.trim().is_empty() {
        if check.must_not_be_empty {
            return Err(SuvError {
                kind: Kind::UserInputErr(std::io::Error::last_os_error()),
                description: "Leere Antworten sind hier nicht erlaubt.".to_string(),
            });
        }
        Ok(check.default_value.clone())
    } else {
        Ok(line.trim().to_string())
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

/// Frage die
// pub(crate) fn ask_option(question: &str) -> Cmd {
//     for cmd in COMMANDS.cmds {
//         println!("{}", cmd.get_option_string());
//     }
//     println!();
//     let s = get_user_input(question, "").trim().to_lowercase();

//     if let Some(res) = COMMANDS.get_by_code(&s) {
//         return res.to_owned().to_owned();
//     }
//     panic!("@todo, Command not known, needs programming");
// }

pub(crate) fn ask_sub_option(question: &str, level: u8, code_filter: &str) -> Cmd {
    let options = COMMANDS.cmds.iter().filter(|c| {
        c.option_level == level
            && (level == 1 || c.option_code.starts_with(&format!("{}-", code_filter)))
    });
    // Welche Optionen (Befehle) gibt es?
    for cmd in options {
        let s = cmd.get_option_string();
        if cmd.option_level > 1 {
            println!("{}", &s[s.find("-").unwrap() + 1..]);
        } else {
            println!("{}", s);
        }
    }
    println!();

    let s = match level > 1 {
        // @todo InputCheck ist nur zum Kompilierbarmachen ausgefüllt, needs thought
        true => {
            format!(
                "{}-{}",
                code_filter,
                get_user_input(
                    question,
                    &InputCheck {
                        must_not_be_empty: true,
                        default_value: "".to_string(),
                        check_format: None,
                    }
                )
                .unwrap()
                .trim()
                .to_lowercase(),
            )
        }
        _ => format!(
            "{}",
            // @todo: InputCheck needs thinking
            get_user_input(
                question,
                &InputCheck {
                    must_not_be_empty: true,
                    default_value: "".to_string(),
                    check_format: None
                }
            )
            .unwrap()
            .trim()
            .to_lowercase()
        ),
    };

    if let Some(res) = COMMANDS.get_by_code(&s) {
        return res.to_owned().to_owned();
    }
    panic!("@todo, Command not known, needs programming");
}
