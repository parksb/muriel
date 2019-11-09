use std::fs::{self};
use crate::util::{get_user_input, load_book_file};
use crate::lib::Book;

pub fn run(id: &str) {
    let book: Book = load_book_file(id);
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
