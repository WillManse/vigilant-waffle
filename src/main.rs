#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read string");
    your_name
}

fn main() {
    let visitor_list = ["bert", "steve", "fred"];
    println!("Hello, what`s your name?");
    let mut allow_them_in = false;
    let name = what_is_your_name();
    for visitor in &visitor_list {
        if visitor == &name {
            allow_them_in = true;
        }
    }

    if allow_them_in {
        println!("Come in");
    } else {
        println!("Stay away");
    }
}
