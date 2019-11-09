use clap::{App, Arg, ArgMatches};
use crate::flags;
use crate::lib::BookField;

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
        .subcommand(
            App::new("update")
                .about("Update a book")
                .arg(Arg::with_name("id").short("i").takes_value(true))
                .arg(Arg::with_name("author").long("author").multiple(true))
                .arg(Arg::with_name("title").long("title").multiple(true))
                .arg(Arg::with_name("publisher").long("publisher"))
                .arg(Arg::with_name("published_at").long("published-at"))
                .arg(Arg::with_name("pages").long("pages"))
                .arg(Arg::with_name("page_at").short("p"))
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
    } else if let Some(ref matches) = matches.subcommand_matches("update") {
        let id = matches.value_of("id").unwrap();

        if matches.is_present("author") {
            flags::update::run(id, BookField::Author);
        }
        if matches.is_present("title") {
            flags::update::run(id, BookField::Title);
        }
        if matches.is_present("publisher") {
            flags::update::run(id, BookField::Publisher);
        }
        if matches.is_present("published_at") {
            flags::update::run(id, BookField::PublishedAt);
        }
        if matches.is_present("pages") {
            flags::update::run(id, BookField::Pages);
        }
        if matches.is_present("page_at") {
            flags::update::run(id, BookField::PageAt);
        }
    }
}
