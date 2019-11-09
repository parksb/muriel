use clap::{App, Arg, ArgMatches};
use crate::flags;

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
        flags::add::run();
    } else if matches.is_present("list") {
        flags::list::run();
    } else if matches.is_present("remove") {
        flags::remove::run(matches.value_of("remove").unwrap());
    }
}
