use std::fs::{self};
use crate::util::{read_data};

pub fn run() {
    println!(
        "\x1b[1;36m{0: <5} {1: <15} {2: <20} {3: <15} {4: <15} {5: <10}\x1b[0m",
        "id", "author", "title", "publisher", "published at", "progress"
    );

    let paths = fs::read_dir("./data").expect("Failed to read directory");
    for path in paths {
        let book = read_data(path.unwrap().path().into_os_string().to_str().unwrap());

        let mut progress_str = String::from("[");
        let progress = (book.page_at as f32 / book.pages as f32) * 100.0;
        for i in 0..10 {
            if i < (progress / 10.0) as i32 {
                progress_str.push_str("=");
            } else {
                progress_str.push_str(" ");
            }
        }
        progress_str.push_str("]");

        println!(
            "{0: <5} {1: <15} {2: <20} {3: <15} {4: <15} {5: <10} {6:.0}%",
            book.id, book.author, book.title, book.publisher, book.published_at, progress_str, progress as f32,
        );
    }
}
