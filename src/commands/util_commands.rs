use std::env::Args;

use crate::{config::SuvFolder, utils::globals::VERSION};

pub(crate) fn stop_suv(_: &Args, _: &SuvFolder) -> bool {
    std::process::exit(0)
}

pub(crate) fn show_version(_: &Args, _: &SuvFolder) -> bool {
    print!(
        "\n\n----------------------------------------\nSUV Version {}\n----------------------------------------\n\n",
        VERSION
    );
    true
}
