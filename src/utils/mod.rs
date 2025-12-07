use std::io;

use chrono::NaiveDate;

/// Minimal way to ask the user for input
/// on a terminal
pub(crate) fn get_user_input(question: &str, default: &str) -> String {
    println!("suv -> {}", question);

    if !default.is_empty() {
        println!("(Empty for `{}`)", &default);
    }

    let mut line = String::from(" ");

    io::stdin()
        .read_line(&mut line)
        .expect("Something went wrong trying to read your input"); // @todo

    if line.trim().is_empty() {
        return default.to_string();
    } else {
        return line.trim().to_string();
    }
}

pub(crate) fn get_optional_user_date(
    question: &str,
    default: &Option<NaiveDate>,
) -> Option<NaiveDate> {
    println!("suv -> {}", question);

    return None;
    // @todo
    // if !default.is_empty() {
    //     println!("(Empty for `{}`)", &default);
    // }

    // let mut line = String::from(" ");

    // io::stdin()
    //     .read_line(&mut line)
    //     .expect("Something went wrong trying to read your input"); // @todo

    // if line.trim().is_empty() {
    //     return default.to_string();
    // } else {
    //     return line.trim().to_string();
    // }
}

pub(crate) fn get_yes_no_user_input(question: &str, default: &bool) -> bool {
    println!("suv -> {}", question);

    let mut line = String::from(" ");

    io::stdin()
        .read_line(&mut line)
        .expect("Something went wrong trying to read your input"); // @todo

    if line.trim().is_empty() {
        return *default;
    } else {
        return line.parse().unwrap_or_default(); // <-- @todo
    }
}
pub(crate) fn ask_option(question: &str, options: &Vec<String>) -> String {
    for (index, option) in options.iter().enumerate() {
        println!("{index} -- {option}");
    }
    println!("");
    let s = get_user_input(question, &"").trim().to_lowercase();
    match s.parse::<usize>() {
        Ok(u) => match options.get(u) {
            Some(response) => return response.to_string(),
            None => panic!("Not understood, @todo needs programming"),
        },
        Err(e) => panic!("@todo, needs programming {e}"),
    }
}
