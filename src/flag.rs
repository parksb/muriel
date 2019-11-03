use std::fs::{self, File};
use std::io::Write;
use crate::lib::Book;
use crate::util::{get_user_input, read_data};

pub fn add() {
    println!("📖 Add new book\n");

    let author = get_user_input("Author");
    let title = get_user_input("Title");
    let publisher = get_user_input("Publisher");
    let publised_at = get_user_input("Published at");
    let pages = get_user_input("Pages");

    let book: Book = Book::new(author, title, publisher, publised_at, pages.parse().unwrap());
    println!("\n✅ {:#?}", book);

    let mut filepath = String::new();
    filepath.push_str(&format!("./data/{}", book.id.to_string()));

    let mut fileheader = String::new();
    fileheader.push_str(&format!(
        "id:{}\nauthor:{}\ntitle:{}\npublisher:{}\npublished_at:{}\npages:{}\npage_at:{}",
        book.id.to_string(), book.author, book.title, book.publisher, book.published_at, pages, book.page_at,
    ));

    let mut file = File::create(&filepath).expect("Failed to create file");
    file.write_all(fileheader.as_bytes()).expect("Failed to write file");
}

pub fn list() {
    println!("📖 List of the books\n");
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

pub fn remove(id: &str) {
    println!("📖 Remove a book\n");

    let book = read_data(&format!("{}/{}", "./data", id));
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
