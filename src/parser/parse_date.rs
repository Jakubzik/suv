use chrono::{Days, NaiveDate};

use crate::{
    parser::date_calculation::{
        get_current_month, get_current_weekday, get_year, read_as_day, read_as_month, today,
    },
    utils::FlexibleDate,
};

impl FlexibleDate {
    // Die Funktion soll die Nutzereingaben *und* die in den
    // Markdown-Dateien gespeicherten Datumswerte interpretieren.
    // Interpretierbar sollen z.B. folgende Werte sein:
    //
    // 13.4.2026
    // April 2026 (*)
    // March 2026
    // März 2026
    // Mittwoch (**)
    // Thursday (**)
    //
    // (*) Vage Angaben (nur der Monat) werden intern
    //     als erster d. Monats abgelegt. Im Bsp.
    //     April 2026 also als 1.4.2026.
    //     Das FlexibleDate hat dann zusätzlich die
    //     Eigenschaft "month_only", damit erkennbar
    //     bleibt, dass der 1.4.2026 nur generell
    //     April 2026 meint
    //
    // (**) Betrifft nur Eingaben der Nutzerin: Wird z.B. "Mittwoch"
    //      eingegeben, berechnet die Funktion das Datum des
    //      auf den Zeitpunkt der Eingabe folgenden Mittwochs.
    //
    // Weiter unten in dieser Datei gibt's ein paar Tests, die
    // die Anwendung der Funktion verdeutlichen.
    pub fn from_str_future(st: &str) -> FlexibleDate {
        if st.trim().is_empty() {
            return FlexibleDate::new_empty();
        }
        let words: Vec<&str> = st.split_whitespace().collect();
        if let Some(mon) = read_as_month(words[0]) {
            let mut i_next_year = 0;
            if words.len() > 1
                && let Ok(yr) = words[1].parse::<usize>()
            {
                i_next_year = yr;
            }

            if i_next_year == 0 && mon <= get_current_month() as usize {
                i_next_year = get_year(1) as usize;
            }

            return FlexibleDate {
                datum: Some(
                    NaiveDate::parse_from_str(&format!("1.{}.{}", mon, i_next_year), "%d.%m.%Y")
                        .unwrap(),
                ), // <- @todo (unwrap)
                month_only: true,
                is_parsed: true,
                input: st.to_string(),
            };
        }

        if let Some(day) = read_as_day(st) {
            // let mut i_next_day = day.abs_diff(get_current_weekday() as usize);
            let mut i_next_day: i32 = day as i32 - (get_current_weekday() as i32);
            if day <= get_current_weekday() as usize {
                i_next_day += 7;
            }

            return FlexibleDate {
                datum: Some(
                    today()
                        .checked_add_days(Days::new(i_next_day as u64))
                        .unwrap(),
                ),
                month_only: false,
                is_parsed: true,
                input: st.to_string(),
            };
        }
        FlexibleDate {
            datum: None,
            month_only: false,
            is_parsed: false,
            input: st.to_string(),
        }
    }
}
#[cfg(test)]
mod test_parsing {
    use chrono::NaiveDate;

    // use crate::utils::globals::get_current_month;
    use crate::{
        parser::date_calculation::{get_current_month, get_year},
        utils::FlexibleDate,
    };

    #[test]
    fn parsing_month() {
        let s_test_1 = "jan";
        let fd_1 = FlexibleDate {
            datum: Some(
                NaiveDate::parse_from_str(&format!("1.1.{}", get_year(1)), "%d.%m.%Y").unwrap(),
            ),
            month_only: true,
            is_parsed: true,
            input: "jan".to_string(),
        };
        let s_test_2 = "Dezember";
        let fd_2 = match get_current_month() {
            12 => FlexibleDate {
                datum: Some(
                    NaiveDate::parse_from_str(&format!("1.12.{}", get_year(1)), "%d.%m.%Y")
                        .unwrap(),
                ),
                month_only: true,
                is_parsed: true,
                input: s_test_2.to_string(),
            },
            _ => FlexibleDate {
                datum: Some(
                    NaiveDate::parse_from_str(&format!("1.12.{}", get_year(1)), "%d.%m.%Y")
                        .unwrap(),
                ),
                month_only: true,
                is_parsed: true,
                input: s_test_2.to_string(),
            },
        };
        let s_test_3 = "Brot";
        let fd_3 = FlexibleDate {
            datum: None,
            month_only: false,
            is_parsed: false,
            input: s_test_3.to_string(),
        };
        let s_test_4 = "Dezember 2027";
        let fd_4 = FlexibleDate {
            datum: Some(NaiveDate::parse_from_str("1.12.2027", "%d.%m.%Y").unwrap()),
            month_only: true,
            is_parsed: true,
            input: s_test_4.to_string(),
        };
        let s_test_5 = "März";
        let fd_5 = FlexibleDate {
            datum: Some(
                NaiveDate::parse_from_str(&format!("1.3.{}", get_year(1)), "%d.%m.%Y").unwrap(),
            ),
            month_only: true,
            is_parsed: true,
            input: s_test_5.to_string(),
        };
        assert_eq!(FlexibleDate::from_str_future(&s_test_1), fd_1);
        assert_eq!(FlexibleDate::from_str_future(&s_test_2), fd_2);
        assert_eq!(FlexibleDate::from_str_future(&s_test_3), fd_3);
        assert_eq!(FlexibleDate::from_str_future(&s_test_4), fd_4);
        assert_eq!(FlexibleDate::from_str_future(&s_test_5), fd_5);
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
