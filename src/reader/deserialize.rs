use std::fs::{self, DirEntry, read_to_string};

use crate::{
    parser::between,
    thesis::{Student, Thesis},
    utils::{FlexibleDate, files_n_folders::file_exists},
};

pub fn deserialize_theses(config: &str) -> Vec<Thesis> {
    let paths = fs::read_dir(config).unwrap();
    let mut theses: Vec<Thesis> = vec![];

    for path in paths {
        if path.as_ref().unwrap().file_type().unwrap().is_dir() {
            if let Some(thesis) = deserialize(&path.unwrap()) {
                theses.push(thesis);
            }
        }
    }
    theses
}

fn deserialize(path: &DirEntry) -> Option<Thesis> {
    // (1) Dateinamen der .md-Datei finden
    // (2) md-Datei öffnen
    // (3) Inhalt der MD Datei interpretieren
    //
    // (1) Rekonstruiere Dateinamen
    let tmp = match &path.file_name().into_string().unwrap().split_once("_") {
        Some((a, b)) => (a.to_string(), b.to_string()),
        _ => ("".to_string(), "".to_string()),
    };

    if tmp.0.is_empty() && tmp.1.is_empty() {
        return None;
    }

    let file = format!("{}/{}_{}.md", path.path().to_str().unwrap(), tmp.0, tmp.1);

    if !file_exists(&file) {
        return None;
    }

    let file_content = read_to_string(&file).unwrap();
    let student = Student {
        last_name: tmp.0,
        first_name: tmp.1,
        email: between(&file_content, "email:", "\n").trim().to_string(),
    };

    Some(Thesis::from_string(student, &file_content))
}
