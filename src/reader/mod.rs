pub(crate) mod deserialize;
use std::fmt::Debug;

use chrono::NaiveDate;

use crate::thesis::Thesis;

pub(crate) struct Step {
    pub(crate) datum_int: Option<NaiveDate>,
    pub(crate) datum_text: String,
    pub(crate) description: String,
}

// Durchsucht alle aktiven Verfahren nach
// Abgabedaten, nächsten Treffen und Schritten,
// sortiert sie chronologisch, und gibt
// eine entsprechende Liste aus
// pub(crate) fn list_next_steps(theses: Vec<Thesis>) {
pub(crate) fn extract_next_steps_from_theses(theses: Vec<Thesis>) -> Vec<Step> {
    // Sammle die Daten -- Anmeldung, Abgabe, nächster Gesprächstermin --
    // in Vec<Step> (um sie dann sortieren zu können)
    let mut steps: Vec<Step> = vec![];
    for thesis in theses {
        if thesis.abgabedatum.is_parsed {
            steps.push(Step {
                datum_int: thesis.abgabedatum.datum,
                datum_text: format!("{:?}", thesis.abgabedatum),
                description: format!("Abgabe '{}' von {}", thesis.title, thesis.student.last_name),
            })
        }
        if thesis.anmeldedatum.is_parsed {
            steps.push(Step {
                datum_int: thesis.anmeldedatum.datum,
                datum_text: format!("{:?}", thesis.anmeldedatum),
                description: format!(
                    "Anmeldung '{}' von {}",
                    thesis.title, thesis.student.last_name
                ),
            })
        }
        if thesis.next_appointment.is_parsed {
            steps.push(Step {
                datum_int: thesis.next_appointment.datum,
                datum_text: format!("{:?}", thesis.next_appointment.datum),
                description: format!("Nächste Verabredung mit {}", thesis.student.last_name),
            })
        }
    }

    // Sortiere die gesammelten Daten
    steps.sort_by(|t1, t2| t1.datum_int.cmp(&t2.datum_int));

    // Schreibe die nächsten anstehenden Termine
    // in chronologischer Reihenfolge auf
    // for step in &steps {
    //     let dtm = match step.datum_int {
    //         Some(nd) => nd.format("%d.%m.%Y").to_string(),
    //         _ => step.datum_text.to_string(),
    //     };
    //     println!("{}\n==========", dtm);
    //     println!("{}\n", step.description.replace("'' von ", ""));
    // }
    steps
}

// // Listet alle Namen von Studierenden auf,
// // deren Verfahren im aktuellen Ordner (und
// // nicht im Archiv) ist
// pub(crate) fn list_current_thesis(mut theses: Vec<Thesis>) {
//     theses.sort_by(|t1, t2| t1.abgabedatum.datum.cmp(&t2.abgabedatum.datum));
//     for thesis in theses {
//         println!(
//             "{} {} ['{}'], Abgabe: {}",
//             thesis.student.first_name, thesis.student.last_name, thesis.title, thesis.abgabedatum,
//         );
//     }
// }
