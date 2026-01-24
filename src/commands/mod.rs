pub(crate) mod add_stuff;
pub(crate) mod read_stuff;
pub(crate) mod util_commands;

// In diesem Modul werden die Funktionen abgelegt,
// die Befehle *ausführen* -- also alles, was
// in CMD.call als Funktionszeiger vorkommt
// (Siehe ui/cmd_interpreter.rs) bzw
// was unmittelbar vom Nutzer in die Wege geleitet wird
//
// Jede der Funktionen hat die Signatur
// fn (&Args, &SuvFolder) -> bool
//

use std::env::Args;

use crate::{
    commands::{
        add_stuff::add_new_thesis,
        read_stuff::{get_reading_options, list_current_thesis, list_next_steps},
        util_commands::{show_version, stop_suv},
    },
    config::SuvFolder,
};

pub(crate) struct Commands {
    // pub(crate) cmds: Vec<Cmd>,
    pub(crate) cmds: &'static [&'static Cmd],
}

#[derive(Clone)]
pub(crate) struct Cmd {
    pub(crate) option_code: &'static str,
    pub(crate) option_level: u8,
    pub(crate) command: &'static [&'static str],
    description: &'static str,
    example_usage: Option<&'static str>,
    pub(crate) call: fn(&Args, &SuvFolder) -> bool,
}

// @todo: check that option-codes are UNIQUE
pub(crate) const fn init_commands() -> &'static Commands {
    &Commands {
        cmds: &[
            &Cmd {
                option_code: "1",
                option_level: 1,
                command: &["add", "new"],
                description: "Neue Betreuung eingeben",
                example_usage: Some("suv add"),
                call: add_new_thesis,
            },
            &Cmd {
                option_code: "2",
                option_level: 1,
                command: &["list", "show"],
                description: "Vorhandene Daten einsehen",
                example_usage: Some("suv list"),
                call: get_reading_options,
            },
            &Cmd {
                option_code: "2-1",
                option_level: 2,
                command: &["list_studierende", "show_studierende"],
                description: "Studierende auflisten",
                example_usage: Some("suv list_studierende"),
                call: list_current_thesis,
            },
            &Cmd {
                option_code: "2-2",
                option_level: 2,
                command: &["list_steps", "show_steps"],
                description: "Nächste Schritte auflisten",
                example_usage: Some("suv list_steps"),
                call: list_next_steps,
            },
            &Cmd {
                option_code: "3",
                option_level: 1,
                command: &["q", "quit", "exit", "stop", "bye"],
                description: "SUV beenden",
                example_usage: None,
                call: stop_suv,
            },
            &Cmd {
                option_code: "4",
                option_level: 1,
                command: &["version", "-v", "--v"],
                description: "Programmversion ausgeben",
                example_usage: None,
                call: show_version,
            },
        ],
    }
}

impl Commands {
    pub(crate) fn get_by_code(&self, option_code: &str) -> Option<&&Cmd> {
        self.cmds.iter().find(|q| q.option_code == option_code)
    }
}

impl Cmd {
    pub(crate) fn get_option_string(&self) -> String {
        format!("{} -- {}", self.option_code, self.description)
    }
}
