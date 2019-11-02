use regex::Regex;
use std::fs::{self, File};
use std::io::{Write, BufReader, BufRead};
use crate::app::get_user_input;
use crate::lib::Book;

pub fn add() {
    println!("📖 Add new book\n");

    let author = get_user_input(String::from("Author"));
    let title = get_user_input(String::from("Title"));
    let publisher = get_user_input(String::from("Publisher"));
    let publised_at = get_user_input(String::from("Published at"));

    let book: Book = Book::new(author, title, publisher, publised_at);
    println!("\n✅ {:#?}", book);

    let mut filepath = String::new();
    filepath.push_str(&(
        "./data/".to_string() +
        &book.id.to_string()
    ));

    let mut fileheader = String::new();
    fileheader.push_str(&(
        "id:".to_string() + &book.id.to_string() + "\n" +
        "author:" + &book.author + "\n" +
        "title:" + &book.title + "\n" +
        "publisher:" + &book.publisher + "\n" +
        "published_at:" + &book.published_at
    ));

    let mut file = File::create(&filepath).expect("Failed to create file");
    file.write_all(fileheader.as_bytes()).expect("Failed to write file");
}

pub fn list() {
    println!("📖 List of the books\n");

    let paths = fs::read_dir("./data").expect("Failed to read directory");
    for path in paths {
        let file = File::open(path.unwrap().path()).expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut lines_iter = reader.lines().map(|l| l.unwrap());
        let regex = Regex::new(r"^\w*:").unwrap();

        println!(
            "\x1b[1;36m{0: <5} {1: <15} {2: <30} {3: <20} {4: <10}\x1b[0m",
            "id", "author", "title", "publisher", "published at"
        );
        println!(
            "{0: <5} {1: <15} {2: <30} {3: <20} {4: <10}",
            regex.replace_all(&lines_iter.next().unwrap(), ""),
            regex.replace_all(&lines_iter.next().unwrap(), ""),
            regex.replace_all(&lines_iter.next().unwrap(), ""),
            regex.replace_all(&lines_iter.next().unwrap(), ""),
            regex.replace_all(&lines_iter.next().unwrap(), ""),
        );
    }
}
