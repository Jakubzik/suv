mod config;
mod parser;
mod reader;
mod thesis;
mod ui;
mod utils;
mod writer;

use crate::{
    config::get_suv_folders, reader::get_reading_options, ui::get_thesis_details_from_user,
    utils::ask_option,
};

// Programmstart:
// 0 - Neue Betreuung?
// 1 - Vorhandene Einsehen?
// 2 - Programm beenden
fn main() {
    let suv_base = get_suv_folders(); // from config or from user.
    let s_options = vec![
        "Neue Betreuung erfassen".to_string(),
        "Vorhandene Daten einsehen".to_string(),
        "SUV beenden".to_string(),
    ];

    let s = ask_option("Was möchtest du machen?", &s_options);
    match s_options.iter().position(|a| a == &s).unwrap() {
        0 => {
            let thesis = get_thesis_details_from_user();
            thesis.store_new(&suv_base.main_directory);
            println!("OK");
        }
        1 => {
            get_reading_options(&suv_base.main_directory);
        }
        2 => {
            println!("Tschüüüsss");
        }
        _ => panic!("Noch nicht programmiert"),
    }
}
