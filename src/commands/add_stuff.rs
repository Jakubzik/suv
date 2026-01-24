use std::env::Args;

use crate::{config::SuvFolder, ui::get_thesis_details_from_user};

/// @todo: auf args reagieren (z.B. Name d. neuen Studierenden)
pub(crate) fn add_new_thesis(_: &Args, conf: &SuvFolder) -> bool {
    println!("\nOk, du möchtest eine neue Betreuung erfassen ...\n");
    let thesis = get_thesis_details_from_user();
    thesis.store_new(&conf.main_directory);
    true
}
