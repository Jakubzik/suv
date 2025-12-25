use crate::parser::between;
use crate::utils::FlexibleDate;

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

    // @todo: Strings als Konstanten ("Registration planned etc.")
    // @todo: prüfen, ob :from_str_future hier nicht zu viel "future" hineininterpretiert?
    pub(crate) fn from_string(student: Student, s_in: &str) -> Thesis {
        Thesis {
            student,
            title: between(&s_in, "title: ", "\n").trim().to_string(),
            abgabedatum: FlexibleDate::from_str_future(
                between(&s_in, "Submission planned: ", "\n").trim(),
            ),
            anmeldedatum: FlexibleDate::from_str_future(
                between(&s_in, "Registration planned: ", "\n").trim(),
            ),
            schnell: between(&s_in, "Submission planned: ", "\n").trim() == "true",
            interesse: between(&s_in, "## Interest\n", "\n#").trim().to_string(),
            steps: between(&s_in, "## Next Steps\n", "\n#").trim().to_string(),
            next_appointment: FlexibleDate::from_str_future(
                between(&s_in, "Next appointment: ", "\n").trim(),
            ),
            todo: between(&s_in, "## To-Do (for me)", "#\n")
                .trim()
                .to_string(),
        }
    }
}
