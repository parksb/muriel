use std::io::{stdin, stdout, Write};

pub fn get_user_input(label: &str) -> String {
    let mut input = String::new();

    print!("{}: ", label);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Failed to read string");

    input[0..input.len() - 1].to_string()
}
