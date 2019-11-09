use std::fs::{self};
use crate::util::{get_user_input};
use crate::lib::Book;

pub fn run(id: &str) {
    let serialized_book = fs::read_to_string(&format!("./data/{}", id)).unwrap();
    let book: Book = serde_json::from_str(&serialized_book).unwrap();
    let book_info = format!("{}, {}, {}, {}", book.author, book.title, book.publisher, book.published_at);

    loop {
        let check = get_user_input(&format!("Are you sure to remove \x1b[1;33m{}\x1b[0m? (y/n)", book_info));
        if check == "y" || check == "yes" {
            fs::remove_file(&format!("./data/{}", id)).expect("Failed to remove file");
            println!("\n🗑 {:#?}", book);
            break;
        } else if check == "n" || check == "no" {
            break;
        }
    }
}
