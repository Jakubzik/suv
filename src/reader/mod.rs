mod deserialize;
use crate::{reader::deserialize::deserialize_theses, thesis::Thesis, utils::ask_option};

pub(crate) fn get_reading_options(config_path: &str) {
    let s_options = vec![
        "Studierende auflisten".to_string(),
        "Nächste Aktivitäten auflisten".to_string(),
        "Archiv durchsuchen [todo]".to_string(),
    ];
    let s = ask_option("Was möchtest du machen?", &s_options);
    let theses: Vec<Thesis> = deserialize_theses(config_path);
    match s_options.iter().position(|a| a == &s).unwrap() {
        0 => {
            list_current_thesis(theses);
        }
        1 => {
            // list_next_steps(&theses);
        }
        2 => {
            println!("@todo ... muss mal programmiert werden");
        }
        _ => panic!("Noch nicht programmiert"),
    }
}

// Durchsucht alle aktiven Verfahren nach
// Abgabedaten, nächsten Treffen und Schritten,
// sortiert sie chronologisch, und gibt
// eine entsprechende Liste aus
fn list_next_steps() {
    todo!()
}

// Listet alle Namen von Studierenden auf,
// deren Verfahren im aktuellen Ordner (und
// nicht im Archiv) ist
fn list_current_thesis(mut theses: Vec<Thesis>) {
    theses.sort_by(|t1, t2| t1.abgabedatum.datum.cmp(&t2.abgabedatum.datum));
    for thesis in theses {
        println!(
            "{} {} ['{}'], Abgabe: {}",
            thesis.student.first_name, thesis.student.last_name, thesis.title, thesis.abgabedatum,
        );
    }
}
