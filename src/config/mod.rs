use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::{
    parser::between,
    utils::{
        InputCheck, get_user_input,
        globals::{CONFIG_FILE, EXIT_CODE_NO_HOME_DIR},
    },
};

#[derive(Debug)]
pub(crate) struct SuvFolder {
    pub(crate) main_directory: String,
    archive_directory: String,
}

/// Get the configured values of
/// the folder containing the supervision files
///
/// If there *is* no config file yet,
/// one is created.
pub(crate) fn get_suv_folders() -> SuvFolder {
    if let Ok(value) = std::env::var("HOME") {
        let config_file = format!("{}/{}", &value, &CONFIG_FILE);
        let home_dir = Path::new(&config_file);
        if home_dir.exists() {
            let cfile = std::fs::read_to_string(home_dir).unwrap();
            let main_directory = between(&cfile, "main_directory=", "\n").to_string();
            let archive_directory = between(&cfile, "archive_directory=", "\n").to_string();
            SuvFolder {
                main_directory,
                archive_directory,
            }
        } else {
            fs::create_dir_all(home_dir.parent().unwrap()).expect("Directory cannot be created.");
            let _ = File::create_new(&config_file).unwrap();
            edit_config()
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
fn edit_config() -> SuvFolder {
    let mut home_dir = String::from("");
    // @todo: das ist bloß qnd, muss nachdenken, was benötigt wird.
    let s_folders = get_suv_folders();
    let ci = InputCheck {
        must_not_be_empty: true,
        default_value: s_folders.main_directory,
        check_format: None,
    };

    let ci_archive = InputCheck {
        must_not_be_empty: true,
        default_value: s_folders.archive_directory,
        check_format: None,
    };
    if let Ok(value) = std::env::var("HOME") {
        home_dir = format!("{}/{}", &value, &CONFIG_FILE);
    } else {
        println!("There's no $HOME set. Sorry, I cannot function under these circumstances");
    }

    // @Fehlermanagement (unwrap!)
    let line_dir =
        get_user_input("Please enter the folder containing the `suv` files:", &ci).unwrap();

    // @Fehlermanagement (unwrap!)
    let line_archive = get_user_input(
        "Please enter the folder containing the `suv` ARCHIVE:",
        &ci_archive,
    )
    .unwrap();

    let ret = SuvFolder {
        main_directory: line_dir.trim().to_string(),
        archive_directory: line_archive.trim().to_string(),
    };

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&home_dir)
        .unwrap();

    let _ = f
        .write(ret.to_config_file_text().as_bytes())
        .expect("Configuration could not be written.");

    ret
}

/// The Config file contains
/// main_directory=/foo/bar... (containing thesis files)
/// archive_directory=/foo/bar2... (to do)
impl SuvFolder {
    fn to_config_file_text(&self) -> String {
        format!(
            "main_directory={}\narchive_directory={}\n", // <- final line break is important for later parsing
            self.main_directory, self.archive_directory
        )
    }
}
