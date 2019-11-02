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
        let book = read_data(path.unwrap().path().into_os_string().to_str().unwrap());

        println!(
            "\x1b[1;36m{0: <5} {1: <15} {2: <30} {3: <20} {4: <10}\x1b[0m",
            "id", "author", "title", "publisher", "published at"
        );
        println!(
            "{0: <5} {1: <15} {2: <30} {3: <20} {4: <10}",
            book.id,
            book.author,
            book.title,
            book.publisher,
            book.published_at,
        );
    }
}

pub fn remove(id: &str) {
    println!("📖 Remove a book\n");

    let book = read_data(&format!("{}/{}", "./data", id));
    let book_info = book.author.to_string() + ", " +
        &book.title + ", " +
        &book.publisher + ", " +
        &book.published_at;

    loop {
        let check = get_user_input(String::from("Are you sure to remove \x1b[1;33m".to_string() + &book_info + "\x1b[0m? (y/n)"));
        if check == "y" || check == "yes" {
            fs::remove_file("./data/".to_string() + id).expect("Failed to remove file");
            println!("\n🗑 {:#?}", book);
            break;
        } else if check == "n" || check == "no" {
            break;
        }
    }
}

fn read_data(path: &str) -> Book {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let regex = Regex::new(r"^\w*:").unwrap();
    let mut lines_iter = reader.lines().map(|l| l.unwrap());

    Book {
        id: regex.replace_all(&lines_iter.next().unwrap(), "").parse().unwrap(),
        author: regex.replace_all(&lines_iter.next().unwrap(), "").to_string(),
        title: regex.replace_all(&lines_iter.next().unwrap(), "").to_string(),
        publisher: regex.replace_all(&lines_iter.next().unwrap(), "").to_string(),
        published_at: regex.replace_all(&lines_iter.next().unwrap(), "").to_string(),
    }
}
