use std::io;
mod globals;

use chrono::{Days, NaiveDate};

use crate::utils::globals::{
    get_current_month, get_current_weekday, get_year, read_as_day, read_as_month, today,
};

// Sometimes you just want
// "January 2026" as Date,
#[derive(PartialEq, Debug)]
pub struct FlexibleDate {
    datum: NaiveDate,
    month_only: bool,
}

impl FlexibleDate {
    // Funktion, die mit Datumsangaben wie "März" (=z.B. Abgabedatum
    // ist der nächste März) oder "Freitag" (=nächster Freitag) umgeht.
    //
    // Parse String with the preconception that it is
    // a future date:
    // `January`, for example, will resolve to the January of the following year
    // `Tuesday` will resolve to the *coming* Tuesday
    // February 2028 will be parsed into 1.2.2028gg
    // (see tests below)
    pub fn from_str_future(st: &str) -> Option<FlexibleDate> {
        let words: Vec<&str> = st.split_whitespace().collect();
        if let Some(mon) = read_as_month(words[0]) {
            let mut i_next_year = 0;
            if words.len() > 1 {
                if let Ok(yr) = words[1].parse::<usize>() {
                    println!("HERE, TOO! : {}", &yr);
                    i_next_year = yr;
                }
            }
            if i_next_year == 0 && mon <= get_current_month() as usize {
                i_next_year = get_year(1) as usize;
            }
            return Some(FlexibleDate {
                datum: NaiveDate::parse_from_str(&format!("1.{}.{}", mon, i_next_year), "%d.%m.%Y")
                    .unwrap(), // <- @todo (unwrap)
                month_only: true,
            });
        }

        if let Some(day) = read_as_day(st) {
            // let mut i_next_day = day.abs_diff(get_current_weekday() as usize);
            let mut i_next_day: i32 = day as i32 - (get_current_weekday() as i32);
            if day <= get_current_weekday() as usize {
                i_next_day += 7;
            }

            return Some(FlexibleDate {
                datum: today()
                    .checked_add_days(Days::new(i_next_day as u64))
                    .unwrap(),
                month_only: false,
            });
        }
        return None;
    }
}

#[cfg(test)]
mod test_parsing {
    use chrono::{Datelike, NaiveDate, Utc};

    use crate::utils::{
        FlexibleDate,
        globals::{get_current_month, get_year},
    };

    #[test]
    fn parsing_month() {
        let s_test_1 = "jan";
        let fd_1 = FlexibleDate {
            datum: NaiveDate::parse_from_str(&format!("1.1.{}", get_year(1)), "%d.%m.%Y").unwrap(),
            month_only: true,
        };
        let s_test_2 = "Dezember";
        let fd_2 = match get_current_month() {
            12 => FlexibleDate {
                datum: NaiveDate::parse_from_str(&format!("1.12.{}", get_year(1)), "%d.%m.%Y")
                    .unwrap(),
                month_only: true,
            },
            _ => FlexibleDate {
                datum: NaiveDate::parse_from_str(&format!("1.12.{}", get_year(1)), "%d.%m.%Y")
                    .unwrap(),
                month_only: true,
            },
        };
        let s_test_3 = "Brot";
        let s_test_4 = "Dezember 2027";
        let fd_4 = FlexibleDate {
            datum: NaiveDate::parse_from_str("1.12.2027", "%d.%m.%Y").unwrap(),
            month_only: true,
        };
        assert_eq!(FlexibleDate::from_str_future(&s_test_1), Some(fd_1));
        assert_eq!(FlexibleDate::from_str_future(&s_test_2), Some(fd_2));
        assert_eq!(FlexibleDate::from_str_future(&s_test_3), None);
        assert_eq!(FlexibleDate::from_str_future(&s_test_4), Some(fd_4));
    }

    #[test]
    fn parsing_day() {
        // Das ist schwer zu testen, hier wird nur
        // die Verwendung angedeutet
        // (Der Test funktionierte nur am 9.12.2025, @todo)
        //
        // let s_test_1 = "Mitt";
        // let s_test_2 = "Monday";
        // let s_test_3 = "Frei";
        // let fd_1 = FlexibleDate {
        //     datum: NaiveDate::parse_from_str("10.12.2025", "%d.%m.%Y").unwrap(),
        //     month_only: false,
        // };
        // let fd_2 = FlexibleDate {
        //     datum: NaiveDate::parse_from_str("15.12.2025", "%d.%m.%Y").unwrap(),
        //     month_only: false,
        // };
        // // let fd_3 = FlexibleDate {
        // //     datum: NaiveDate::parse_from_str("12.12.2025", "%d.%m.%Y").unwrap(),
        // //     month_only: false,
        // // };
        // assert_eq!(FlexibleDate::from_str_future(&s_test_1), Some(fd_1));
        // assert_eq!(FlexibleDate::from_str_future(&s_test_2), Some(fd_2));
        // assert_eq!(FlexibleDate::from_str_future(&s_test_3), None);
    } // use super::*;
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
        return default.to_string();
    } else {
        return line.trim().to_string();
    }
}

pub(crate) fn get_optional_user_date(
    question: &str,
    default: &Option<NaiveDate>,
) -> Option<NaiveDate> {
    println!("suv -> {}", question);

    return None;
    // @todo
    // if !default.is_empty() {
    //     println!("(Empty for `{}`)", &default);
    // }

    // let mut line = String::from(" ");

    // io::stdin()
    //     .read_line(&mut line)
    //     .expect("Something went wrong trying to read your input"); // @todo

    // if line.trim().is_empty() {
    //     return default.to_string();
    // } else {
    //     return line.trim().to_string();
    // }
}

pub(crate) fn get_yes_no_user_input(question: &str, default: &bool) -> bool {
    println!("suv -> {}", question);

    let mut line = String::from(" ");

    io::stdin()
        .read_line(&mut line)
        .expect("Something went wrong trying to read your input"); // @todo

    if line.trim().is_empty() {
        return *default;
    } else {
        return line.parse().unwrap_or_default(); // <-- @todo
    }
}
pub(crate) fn ask_option(question: &str, options: &Vec<String>) -> String {
    for (index, option) in options.iter().enumerate() {
        println!("{index} -- {option}");
    }
    println!("");
    let s = get_user_input(question, &"").trim().to_lowercase();
    match s.parse::<usize>() {
        Ok(u) => match options.get(u) {
            Some(response) => return response.to_string(),
            None => panic!("Not understood, @todo needs programming"),
        },
        Err(e) => panic!("@todo, needs programming {e}"),
    }
}
