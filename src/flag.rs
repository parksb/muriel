use std::fs::File;
use std::io::Write;
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
        &book.author + ", " +
        &book.title + ", " +
        &book.publisher + ", " +
        &book.published_at
    ));

    let mut fileheader = String::new();
    fileheader.push_str(&(
        "author:".to_string() + &book.author + "\n" +
        "title:" + &book.title + "\n" +
        "publisher:" + &book.publisher + "\n" +
        "published_at:" + &book.published_at
    ));

    let mut file = File::create(&filepath).expect("Failed to create file");
    file.write_all(fileheader.as_bytes()).expect("Failed to write file");
}
