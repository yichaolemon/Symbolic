mod parser;

use std::io;

fn main() {
    let mut expr = String::new();

    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read expression");

    println!("Hello world!");
    parser::parse(expr);
}
