use chrono::Days;
pub(crate) const EXIT_CODE_NO_HOME_DIR: i32 = 1;
// const VERSION: &str = "0.0.2, Nov 7, 2025 (hj)";
// const VERSION: &str = "0.0.3, Dec 5, 2025 (hj)";
pub(crate) const VERSION: &str = "0.0.4, Dec 14, 2025 (hj)";
pub(crate) const CONFIG_FILE: &str = ".config/suv/suv.rc";

pub(crate) const TIME_FOR_BA_PAPER_PRODUCTION: Days = Days::new(63);
pub(crate) const MONTHS: &[&str] = &[
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub(crate) const DAYS: &[&str] = &["mon", "tue", "wed", "thu", "fri", "sat", "sun"];

pub(crate) const MONATE: &[&str] = &[
    "Januar",
    "Februar",
    "März",
    "April",
    "Mai",
    "Juni",
    "Juli",
    "August",
    "September",
    "Oktober",
    "November",
    "Dezember",
];

pub(crate) const TAGE: &[&str] = &["mon", "die", "mit", "don", "fri", "sam", "son"];
