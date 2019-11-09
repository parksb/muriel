use crate::util::{get_user_input, load_book_file, save_book_file};
use crate::lib::{Book, BookField};

pub fn run(id: &str, field: BookField) {
    let mut book: Book = load_book_file(id);

    match field {
        BookField::Author => {
            let new_value = get_user_input("New author");
            book.set_author(new_value);
        },
        BookField::Title => {
            let new_value = get_user_input("New title");
            book.set_title(new_value);
        },
        BookField::Publisher => {
            let new_value = get_user_input("New publisher");
            book.set_publisher(new_value);
        },
        BookField::PublishedAt => {
            let new_value = get_user_input("New published_at");
            book.set_published_at(new_value);
        },
        BookField::Pages => {
            let new_value = get_user_input("New pages");
            book.set_pages(new_value.parse().unwrap());
        },
        BookField::PageAt => {
            let new_value = get_user_input("New page_at");
            book.set_page_at(new_value.parse().unwrap());
        },
    }

    save_book_file(&book);
}
