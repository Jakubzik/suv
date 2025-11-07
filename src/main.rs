mod parser;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use crate::parser::between;

// Linux only
const EXIT_CODE_NO_HOME_DIR: i32 = 1;
const VERSION: &str = "0.0.2, Nov 7, 2025 (hj)";
const CONFIG_FILE: &str = ".config/suv.suv.rc";

fn main() {
    let suv_base = get_suv_folders(); // from config or from user.
    println!("Not much programmed, yet.");
    println!("We have a conf-file, though: `{}`", &CONFIG_FILE);
    println!("And its content: {:?}", &suv_base);
}

#[derive(Debug)]
struct SUV_FOLDER {
    main_directory: String,
    archive_directory: String,
}

struct Student {
    last_name: String,
    first_name: String,
    email: String,
}

struct Thesis {
    student: Student,
    title: String,
}

/// Get the configured values of
/// the folder containing the supervision files
///
/// If there *is* no config file yet,
/// one is created.
fn get_suv_folders() -> SUV_FOLDER {
    if let Ok(value) = std::env::var("HOME") {
        let config_file = format!("{}/{}", &value, &CONFIG_FILE);
        let home_dir = Path::new(&config_file);
        if home_dir.exists() {
            let cfile = std::fs::read_to_string(home_dir).unwrap();
            let main_directory = between(&cfile, "main_directory=", "\n").to_string();
            let archive_directory = between(&cfile, "archive_directory=", "\n").to_string();
            SUV_FOLDER {
                main_directory,
                archive_directory,
            }
        } else {
            fs::create_dir_all(home_dir.parent().unwrap()).expect("Directory cannot be created.");
            let _ = File::create_new(&config_file).unwrap();
            return edit_config();
        }
    } else {
        println!(
            "What OS is this? Cannot find $HOME, which I need to locate the configuration file"
        );
        std::process::exit(EXIT_CODE_NO_HOME_DIR);
    }
}

/// Ask for directory containing the suv-files,
/// and for directory containing the archive.
fn edit_config() -> SUV_FOLDER {
    let mut home_dir = String::from("");

    if let Ok(value) = std::env::var("HOME") {
        home_dir = format!("{}/{}", &value, &CONFIG_FILE);
    } else {
        println!("There's no $HOME set. Sorry, I cannot function under these circumstances");
    }

    let s_folders = get_suv_folders();
    let s = s_folders.main_directory;

    let line_dir = get_user_input("Please enter the folder containing the `suv` files:", &s);

    let line_archive = get_user_input(
        "Please enter the folder containing the `suv` ARCHIVE:",
        &s_folders.archive_directory,
    );

    let ret = SUV_FOLDER {
        main_directory: line_dir.trim().to_string(),
        archive_directory: line_archive.trim().to_string(),
    };

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&home_dir)
        .unwrap();

    f.write(ret.to_config_file_text().as_bytes())
        .expect("Configuration could not be written.");

    ret
}

/// Minimal way to ask the user for input
/// on a terminal
fn get_user_input(question: &str, default: &str) -> String {
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

/// The Config file contains
/// main_directory=/foo/bar... (containing thesis files)
/// archive_directory=/foo/bar2... (to do)
impl SUV_FOLDER {
    fn to_config_file_text(&self) -> String {
        format!(
            "main_directory={}\narchive_directory={}\n", // <- final line break is important for later parsing
            self.main_directory, self.archive_directory
        )
    }
}
