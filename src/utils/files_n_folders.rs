use std::{fs, path::Path};

use crate::thesis::Student;

// Falls eine These schon angelegt ist, erkennen
// wir das (hoffentlich) am vorhandenen Ordner.
//
// Für diesen Zweck existiert der
// Ordner schon, wenn er in Kleinbuchstaben
// vorhanden ist
pub(crate) fn folder_exists(s_path: &str) -> bool {
    // let p = Path::new(s_path);
    // let folder_name = p.iter().last().unwrap();
    fs::exists(s_path).unwrap_or(true)
}

pub(crate) fn file_exists(s_path: &str) -> bool {
    folder_exists(s_path)
}

pub(crate) fn get_thesis_folder(stud: &Student, config_path: &str) -> Option<String> {
    let folder = format!(
        "{}/{}_{}",
        &config_path,
        stud.last_name.to_lowercase(),
        stud.first_name.to_lowercase()
    );
    match folder_exists(&folder) {
        true => None,
        _ => Some(folder),
    }
}

/// @deprecated
pub(crate) fn get_names_and_titles(config_path: &str) -> Vec<String> {
    let folder = Path::new(config_path);
    folder
        .read_dir()
        .unwrap()
        .map(|student| parse_name(&student.unwrap()))
        .collect()
    // vec![]
}

/// @deprecated
pub(crate) fn parse_name(name: &fs::DirEntry) -> String {
    let tmp = name.file_name().into_string().unwrap();

    let first_last = tmp.split_once("_").unwrap(); // @todo unwrapping!

    let first = &first_last.0.to_string();
    let last = first_last.1.to_string().replace(".md", "");

    format!("{} {}", first, last)
}
