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
        .arg(
            Arg::with_name("list")
                .long("list")
                .short("l")
                .help("List all books")
        )
        .arg(
            Arg::with_name("remove")
                .long("remove")
                .takes_value(true)
                .help("Remove a book")
        )
        .get_matches()
}

pub fn run(matches: ArgMatches) {
    if matches.is_present("add") {
        flag::add();
    } else if matches.is_present("list") {
        flag::list();
    } else if matches.is_present("remove") {
        flag::remove(matches.value_of("remove").unwrap());
    }
}
