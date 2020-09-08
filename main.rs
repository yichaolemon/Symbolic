mod parser;

use std::io;

fn main() {
	let mut expr = String::new();

	io::stdin()
		.read_line(&mut expr)
		.expect("Failed to read expression");

	println!("Hello world!");
	match parser::parse(expr) {
		Ok(e) => println!("Parsed expression: {}", e),
		Err(e) => panic!("err: {}", e),
	}
}
