use std::fs::{File};
use std::io::Write;
use crate::lib::Book;
use crate::util::{get_user_input};

pub fn run() {
    let author = get_user_input("Author");
    let title = get_user_input("Title");
    let publisher = get_user_input("Publisher");
    let publised_at = get_user_input("Published at");
    let pages = get_user_input("Pages");

    let book: Book = Book::new(author, title, publisher, publised_at, pages.parse().unwrap());
    let serialized_book = serde_json::to_string(&book).unwrap();
    println!("\n✅ {:#?}", book);

    let mut filepath = String::new();
    filepath.push_str(&format!("./data/{}", book.id.to_string()));

    let mut fileheader = String::new();
    fileheader.push_str(&serialized_book);

    let mut file = File::create(&filepath).expect("Failed to create file");
    file.write_all(fileheader.as_bytes()).expect("Failed to write file");
}
