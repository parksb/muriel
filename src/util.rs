use std::io::{stdin, stdout, Write};
use std::fs::{read_to_string, write};
use crate::lib::Book;

pub fn get_user_input(label: &str) -> String {
    let mut input = String::new();

    print!("{}: ", label);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Failed to read string");

    input[0..input.len() - 1].to_string()
}

pub fn load_book_file(id: &str) -> Book {
    let filepath = format!("./data/{}", id);
    let serialized_book = read_to_string(filepath).expect("Failed to serialize the data");
    serde_json::from_str(&serialized_book).expect("Failed to deserialize the data")
}

pub fn save_book_file(book: &Book) {
    let filepath = format!("./data/{}", book.id);
    let serialized_book = serde_json::to_string(&book).expect("Failed to serialize the data");
    write(filepath, serialized_book.as_bytes()).expect("Failed to write file");
}
