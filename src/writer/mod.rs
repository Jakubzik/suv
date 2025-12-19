use core::fmt;
use std::{
    fs::{self, File},
    io::Write,
    process::exit,
};

use crate::{thesis::Thesis, utils::files_n_folders::get_thesis_folder};

// Hier ist das Format für die Markdown
// Ausgabedatei festgelegt.
//
// Die Werte unterhalb von "# Thesis" sollen
// möglichst automatisch einlesbar und auswertbar
// sein (damit man später Fragen stellen kann wie:
// wie viele BA-Arbeiten werden im Februar fällig).
//
// Für suv verständliche Datumswerte finden sich
// in ./parser/parse_date.rs
//
// # Thesis Vorname Nachname
// Email: addr
// Submission planned: ...
// Registration planned: ...
// ...
// ## To-Do (for me)
// ...
//
impl fmt::Display for Thesis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text_lines: Vec<String> = vec![];
        text_lines.push(format!(
            "# Thesis {} {}\n\n",
            self.student.first_name, self.student.last_name
        ));
        text_lines.push(format!("Email: {}\n", self.student.email));
        text_lines.push(format!("Submission planned: {}\n", self.abgabedatum));
        text_lines.push(format!("Registration planned: {}\n", self.anmeldedatum));
        text_lines.push(format!("Urgent grading? {}\n", self.schnell));
        text_lines.push(format!("Next appointment: {}\n", self.next_appointment));
        text_lines.push(format!("\n\n## To-Do (for me)\n\n{}", self.todo));
        text_lines.push(format!("\n\n## Interest\n\n{}", self.interesse));
        text_lines.push(format!("\n\n## Next Steps\n\n{}", self.steps));
        write!(f, "{}", text_lines.join("\n"))
    }
}

impl Thesis {
    // Hier wird die Datei der Thesis gespeichert,
    // und zwar in PFAD/nachname_vorname/nachname_vorname.md
    //
    // "PFAD" ist dabei der in der Konfigurationsdatei
    // festgelegte Ordner.
    pub(crate) fn store_new(&self, config_path: &str) {
        if let Some(folder) = get_thesis_folder(&self.student, config_path) {
            fs::create_dir_all(&folder).expect("Kann Ordner nicht erstellen :-(");
            let file_path = format!(
                "{}/{}_{}/{}_{}.md",
                config_path,
                self.student.last_name,
                self.student.first_name,
                self.student.last_name,
                self.student.first_name
            );

            let mut file = File::create(&file_path).expect("Konnte Datei nicht erstellen");

            file.write_all(self.to_string().as_bytes())
                .expect("Konnte Text nicht schreiben");
            println!("\n\nOK, Datei >{}< angelegt.", &file_path);
        } else {
            println!(
                "Achtung, diese BA-Arbeit gibt's offenbar schon. Bitte erst den Ordner löschen, bevor eine neue These mit diesem Namen angelegt wird."
            );
            exit(1);
        }
    }
}
