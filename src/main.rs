mod commands;
mod config;
mod parser;
mod reader;
mod thesis;
mod ui;
mod utils;
mod writer;

use crate::{
    commands::Cmd,
    config::get_suv_folders,
    utils::{ask_sub_option, globals::COMMANDS},
};

// Programmstart:
// 1 - Neue Betreuung?
// 2 - Vorhandene Einsehen?
//   |
//    - 2-1 Studierende auflisten
//    - 2-2 Nächste Aktivitäten auflisten
//    - 2-3 Archiv durchsuchen (@todo)
// 3 - Programm beenden
// 4 - Programmversion nennen
fn main() {
    let suv_base = get_suv_folders(); // from config or from user.

    let cmd = match std::env::args().nth(1) {
        Some(arg1) => get_command(&arg1),
        _ => ask_sub_option("Was möchtest du machen?", 1, ""),
    };

    (cmd.call)(&std::env::args(), &suv_base);
}

// Nutzereingabe soll Funktion, Beschreibung etc. liefern
//
// get_command("new") -> Cmd{option_code: 1, ...}
fn get_command(user_input: &str) -> Cmd {
    match COMMANDS
        .cmds
        .iter()
        .find(|b| b.command.contains(&user_input))
    {
        Some(command) => command.to_owned().to_owned(),
        None => {
            println!("\n\nSorry, Befehl `{}` verstehe ich nicht.", user_input);
            std::process::exit(1);
        }
    }
}
