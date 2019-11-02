use std::io::{stdin, stdout, Write};
use clap::{App, Arg, ArgMatches};
use crate::flag;

pub fn build() -> ArgMatches<'static> {
    App::new("muriel")
        .arg(
            Arg::with_name("add")
                .long("add")
                .short("a")
                .help("Add a new book")
        )
        .get_matches()
}

pub fn run(matches: ArgMatches) {
    if matches.is_present("add") {
        flag::add();
    }
}

pub fn get_user_input(label: String) -> String {
    let mut input = String::new();

    print!("{}: ", label);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Failed to read string");

    return input[0..input.len() - 1].to_string();
}
