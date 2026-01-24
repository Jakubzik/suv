use crate::{
    config::SuvFolder,
    reader::{deserialize::deserialize_theses, extract_next_steps_from_theses},
    thesis::Thesis,
    utils::ask_sub_option,
};
use std::env::Args;

pub(crate) fn get_reading_options(args: &Args, conf: &SuvFolder) -> bool {
    let s = ask_sub_option("Was soll aufgelistet werden??", 2, "2");
    (s.call)(args, conf);
    true
}
pub(crate) fn list_next_steps(_: &Args, conf: &SuvFolder) -> bool {
    let theses: Vec<Thesis> = deserialize_theses(&conf.main_directory);
    let steps = extract_next_steps_from_theses(theses);
    for step in &steps {
        let dtm = match step.datum_int {
            Some(nd) => nd.format("%d.%m.%Y").to_string(),
            _ => step.datum_text.to_string(),
        };
        println!("{}\n==========", dtm);
        println!("{}\n", step.description.replace("'' von ", ""));
    }
    true
}

// Listet alle Namen von Studierenden auf,
// deren Verfahren im aktuellen Ordner (und
// nicht im Archiv) ist
// pub(crate) fn list_current_thesis(mut theses: Vec<Thesis>) {
pub(crate) fn list_current_thesis(_: &Args, conf: &SuvFolder) -> bool {
    let mut theses: Vec<Thesis> = deserialize_theses(&conf.main_directory);
    theses.sort_by(|t1, t2| t1.abgabedatum.datum.cmp(&t2.abgabedatum.datum));
    for thesis in theses {
        println!(
            "{} {} ['{}'], Abgabe: {}",
            thesis.student.first_name, thesis.student.last_name, thesis.title, thesis.abgabedatum,
        );
    }
    true
}
