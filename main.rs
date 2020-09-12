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
			Ok(e) => {
				println!("Parsed expression: {}", e);
				for equivalence in tree_transform::get_transformations().iter() {
					println!("applying equivalence {}", equivalence);
					let transformed_expressions = tree_transform::transform(&e, equivalence);
					for transformed in transformed_expressions.iter() {
						println!("{}", transformed)
					}
				}
			},
			Err(e) => panic!("err: {}", e),
		}
	}
}
