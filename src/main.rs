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
//   |
//    - 0 Studierende auflisten
//    - 1 Nächste Aktivitäten auflisten
//    - 2 Archiv durchsuchen
// 2 - Programm beenden
fn main() {
    let suv_base = get_suv_folders(); // from config or from user.

    let s_options = vec![
        "Neue Betreuung erfassen".to_string(),
        "Vorhandene Daten einsehen".to_string(),
        "SUV beenden".to_string(),
    ];

    let cmd = match std::env::args().nth(1) {
        Some(arg1) => get_command(arg1),
        _ => ask_option("Was möchtest du machen?", &s_options),
    };

    match s_options.iter().position(|a| a == &cmd).unwrap() {
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

fn get_command(arg1: String) -> String {
    match &arg1[..] {
        "add" | "new" => String::from("Neue Betreuung erfassen"),
        "list" | "show" => String::from("Vorhandene Daten einsehen"),
        _ => String::from("SUV beenden"),
    }
}
