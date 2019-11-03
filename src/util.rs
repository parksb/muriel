use regex::Regex;
use std::fs::{File};
use std::io::{stdin, stdout, Write, BufReader, BufRead};
use crate::lib::Book;

pub fn read_data(path: &str) -> Book {
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
        pages: regex.replace_all(&lines_iter.next().unwrap(), "").parse().unwrap(),
        page_at: regex.replace_all(&lines_iter.next().unwrap(), "").parse().unwrap(),
    }
}

pub fn get_user_input(label: &str) -> String {
    let mut input = String::new();

    print!("{}: ", label);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Failed to read string");

    input[0..input.len() - 1].to_string()
}