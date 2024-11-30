use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}

fn use_unwrap() {
    // If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us.
    let greeting_file = File::open("hello.txt").unwrap();
}

fn use_expect() {
    // kind of same as unwrap, intead we can pass custom error message
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn panic_test() {
    let v = vec![1, 2, 3];

    // panic test
    v[99];
}

// propagate errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// shortcut to propagate errors
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_shortest_fn() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Usage of ? with Option
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
