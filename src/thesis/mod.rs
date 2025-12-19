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
    pub(crate) next_appointment: String,
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
            next_appointment: s_empty.clone(),
            todo: s_empty.clone(),
        }
    }
}

impl Thesis {
    pub fn new() -> Self {
        Default::default()
    }
}
