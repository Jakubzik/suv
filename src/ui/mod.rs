use std::{
    fs::{self, File},
    io::Write,
};

use crate::{
    Thesis,
    utils::{get_optional_user_date, get_user_input, get_yes_no_user_input},
};

pub(crate) fn get_thesis_details_from_user() -> Thesis {
    let mut new_thesis = Thesis::new();

    new_thesis.student.last_name = get_user_input("Nachname", "");
    new_thesis.student.first_name = get_user_input("Vorname", "");
    new_thesis.student.email = get_user_input("E-Mail", "");
    new_thesis.abgabedatum = get_optional_user_date("Abgabedatum", &None);
    new_thesis.anmeldedatum = get_optional_user_date("Anmeldedatum", &None);
    new_thesis.schnell = get_yes_no_user_input("Eilt es?", &false);
    new_thesis.interesse = get_user_input("Interesse d. KandidatIn?", "");
    new_thesis.steps = get_user_input("Nächste Schritte d. KandidatIn?", "");
    new_thesis.next_appointment = get_user_input("Nächstes Treffen?", "");
    new_thesis.todo = get_user_input("Meine nächsten Schritte?", "");

    new_thesis
}

impl Thesis {
    fn to_string(&self) -> String {
        let mut text_lines: Vec<String> = vec![];
        text_lines.push(format!(
            "# Thesis {} {}\n\n",
            self.student.first_name, self.student.last_name
        ));
        text_lines.push(format!("Email: {}\n", self.student.email));
        text_lines.push(format!("Submission planned: {:?}\n", self.abgabedatum));
        text_lines.push(format!("Registration planned: {:?}\n", self.anmeldedatum));
        text_lines.push(format!("Urgent grading? {}\n", self.schnell));
        text_lines.push(format!("Next appointment: {}\n", self.next_appointment));
        text_lines.push(format!("\n\n## To-Do (for me)\n\n{}", self.todo));
        text_lines.push(format!("\n\n## Interest\n\n{}", self.interesse));
        text_lines.push(format!("\n\n## Next Steps\n\n{}", self.steps));
        text_lines.join("\n")
    }
    pub(crate) fn store_new(&self, config_path: &str) {
        let folder = format!(
            "{}/{}_{}",
            &config_path, self.student.last_name, self.student.first_name
        );
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
    }
}
