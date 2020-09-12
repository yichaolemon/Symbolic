mod parser;
mod tree_transform;

use std::io;

fn main() {
	loop {
		let mut expr = String::new();

		io::stdin()
			.read_line(&mut expr)
			.expect("Failed to read expression");

		match parser::parse(expr.trim()) {
			Ok(e) => println!("Parsed expression: {}", e),
			Err(e) => panic!("err: {}", e),
		}
	}
}
