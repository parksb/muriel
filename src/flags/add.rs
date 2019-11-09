use crate::lib::Book;
use crate::util::{get_user_input, save_book_file};

pub fn run() {
    let author = get_user_input("Author");
    let title = get_user_input("Title");
    let publisher = get_user_input("Publisher");
    let publised_at = get_user_input("Published at");
    let pages = get_user_input("Pages");
    let book: Book = Book::new(author, title, publisher, publised_at, pages.parse().unwrap());

    save_book_file(&book);

    println!("\n✅ {:#?}", book);
}
