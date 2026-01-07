use crate::parser::between;
use crate::utils::FlexibleDate;
use crate::utils::globals::{
    KEY_INTEREST, KEY_IS_URGENT, KEY_NEXT_APPOINTMENT, KEY_NEXT_STEPS, KEY_REGISTRATION_PLANNED,
    KEY_SUBMISSION_PLANNED, KEY_TITLE, KEY_TO_DO_SELF,
};

pub(crate) struct Student {
    pub(crate) last_name: String,
    pub(crate) first_name: String,
    pub(crate) email: String,
}

impl Default for Student {
    fn default() -> Self {
        Student {
            last_name: "#".to_string(),
            first_name: "#".to_string(),
            email: "".to_string(),
        }
    }
}

impl Student {
    pub fn new() -> Self {
        Default::default()
    }
}

pub(crate) struct Thesis {
    pub(crate) student: Student,
    pub(crate) title: String,
    pub(crate) abgabedatum: FlexibleDate,
    pub(crate) anmeldedatum: FlexibleDate,
    pub(crate) schnell: bool,
    pub(crate) interesse: String,
    pub(crate) steps: String,
    pub(crate) next_appointment: FlexibleDate,
    pub(crate) todo: String,
}

impl Default for Thesis {
    fn default() -> Self {
        let s_empty = "#".to_string();
        Thesis {
            student: Student::new(),
            title: s_empty.clone(),
            abgabedatum: FlexibleDate::new_empty(),
            anmeldedatum: FlexibleDate::new_empty(),
            schnell: false,
            interesse: s_empty.clone(),
            steps: s_empty.clone(),
            next_appointment: FlexibleDate::new_empty(),
            todo: s_empty.clone(),
        }
    }
}

impl Thesis {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    ///
    /// Lange Texte sind im Markdown unter "# Langer Text\n..."
    /// zu finden.
    ///
    /// Kurze Angaben (Titel der BA-Arbeit, Datumsangaben) zwischen
    /// "key: HIER\n"
    ///
    /// Diese Hilfsfunktion liefert jeweils den Wert als String zurück
    ///
    /// @todo: wie geht .between um, wenn das Endzeichen nicht
    /// kommt, sondern das Ende der Datei zuerst erreicht ist?
    pub(crate) fn get_value_of(s_in: &str, key: &str) -> String {
        let f_text = format!("{}\n", key);
        let f_val = format!("{}: ", key);
        match s_in.starts_with("#") {
            true => between(s_in, &f_text, "\n#"),
            false => between(s_in, &f_val, "\n"),
        }
        .trim()
        .to_string()
    }

    // @todo: prüfen, ob :from_str_future hier nicht zu viel "future" hineininterpretiert?
    pub(crate) fn from_string(student: Student, s_in: &str) -> Thesis {
        Thesis {
            student,
            // @todo: Überlegen: mit "\n" als End-Begrenzung funktioniert es u.U. (didn't check) nicht, wenn der Titel in der letzten Zeile steht.
            title: Self::get_value_of(s_in, KEY_TITLE),
            abgabedatum: FlexibleDate::from_str_future(&Self::get_value_of(
                s_in,
                KEY_SUBMISSION_PLANNED,
            )),
            anmeldedatum: FlexibleDate::from_str_future(&Self::get_value_of(
                s_in,
                KEY_REGISTRATION_PLANNED,
            )),
            schnell: Self::get_value_of(s_in, KEY_IS_URGENT) == "true",
            interesse: Self::get_value_of(s_in, KEY_INTEREST),
            steps: Self::get_value_of(s_in, KEY_NEXT_STEPS),
            next_appointment: FlexibleDate::from_str_future(&Self::get_value_of(
                s_in,
                KEY_NEXT_APPOINTMENT,
            )),
            todo: Self::get_value_of(s_in, KEY_TO_DO_SELF),
        }
    }
}
