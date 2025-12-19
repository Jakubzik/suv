use std::fs::{self, DirEntry, read_to_string};

use crate::{
    parser::between,
    thesis::{Student, Thesis},
    utils::{
        FlexibleDate,
        files_n_folders::{file_exists, folder_exists, parse_name},
    },
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
        last_name: tmp.1,
        first_name: tmp.0,
        email: between(&file_content, "email:", "\n").trim().to_string(),
    };

    // @todo: Strings als Konstanten ("Registration planned etc.")
    // @todo: prüfen, ob :from_str_future hier nicht zu viel "future" hineininterpretiert?
    let mut thesis = Thesis {
        student: student,
        title: between(&file_content, "title: ", "\n").trim().to_string(),
        abgabedatum: FlexibleDate::from_str_future(
            between(&file_content, "Submission planned: ", "\n").trim(),
        ),
        anmeldedatum: FlexibleDate::from_str_future(
            between(&file_content, "Registration planned: ", "\n").trim(),
        ),
        schnell: between(&file_content, "Submission planned: ", "\n").trim() == "true",
        interesse: between(&file_content, "## Interest\n", "\n#")
            .trim()
            .to_string(),
        steps: between(&file_content, "## Next Steps\n", "\n#")
            .trim()
            .to_string(),
        next_appointment: FlexibleDate::from_str_future(
            between(&file_content, "Next appointment: ", "\n").trim(),
        )
        .to_string(),
        todo: between(&file_content, "## To-Do (for me)", "#\n")
            .trim()
            .to_string(),
    };

    // println!("{:?} exists? {}", file, folder_exists(&file));
    print!("{}", thesis);
    None
}
