use std::process::exit;

use crate::{
    get_suv_folders,
    parser::date_calculation::get_default_anmelde_from_abgabe,
    thesis::{Student, Thesis},
    utils::{
        InputCheck, files_n_folders::get_thesis_folder, get_optional_user_date, get_user_input,
        get_yes_no_user_input,
    },
};

// Fragt die Nutzerin nach Informationen
// zur These.
// @todo
// - weitere Fragen (z.B. nach dem Titel der Arbeit)
// - Plausibilitätsprüfungen und Rückmeldungen (
//     - Name leer?
//     - E-Mail Adresse plausibel?
//     - Datum verständlich?
// - Möglichkeit zur Korrektur
//     - nach Eingabe alles nochmal anzeigen und
//       Korrekturen ermöglichen? Oder macht man
//       das dann lieber per vim?
pub(crate) fn get_thesis_details_from_user() -> Thesis {
    let mut new_thesis = Thesis::new();

    // @todo: das ist bloß qnd, muss nachdenken, was benötigt wird.
    let ci = InputCheck {
        must_not_be_empty: true,
        default_value: "".to_string(),
        check_format: None,
    };

    // @todo Fehlerbehandlung!
    new_thesis.student.last_name = get_user_input("Nachname", &ci).unwrap();
    new_thesis.student.first_name = get_user_input("Vorname", &ci).unwrap();
    check_if_we_do_not_already_have_a_record(&new_thesis.student);

    new_thesis.student.email = get_user_input("E-Mail", &ci).unwrap();
    new_thesis.zweitgutachter = get_user_input("Zweitgutachter", &ci).unwrap();
    new_thesis.abgabedatum = get_optional_user_date("Abgabedatum", &None);
    new_thesis.anmeldedatum = get_optional_user_date(
        "Anmeldedatum",
        &Some(get_default_anmelde_from_abgabe(
            new_thesis.abgabedatum.clone(),
        )),
    );
    new_thesis.schnell = get_yes_no_user_input("Eilt es?", &false);
    new_thesis.interesse = get_user_input("Interesse d. KandidatIn?", &ci).unwrap();
    new_thesis.steps = get_user_input("Nächste Schritte d. KandidatIn?", &ci).unwrap();
    new_thesis.next_appointment = get_optional_user_date("Nächstes Treffen?", &None);
    new_thesis.todo = get_user_input("Meine nächsten Schritte?", &ci).unwrap();

    new_thesis
}

fn check_if_we_do_not_already_have_a_record(student: &Student) {
    if get_thesis_folder(student, &get_suv_folders().main_directory).is_none() {
        println!("Sorry, dazu gibt es schon eine Datei! Bitte zuerst den Ordner löschen");
        exit(1);
    }
}
