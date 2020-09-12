use std::io;

fn main() {
	loop {
		println!("Enter a mathematical expression");
		let mut expr = String::new();

		io::stdin()
			.read_line(&mut expr)
			.expect("Failed to read expression");

		match parser::parse(expr.trim()) {
			Ok(e) => {
				println!("Parsed expression: {}", e);
				for equivalence in tree_transform::get_transformations().iter() {
					let transformed_expressions = tree_transform::transform(&e, equivalence);
					for transformed in transformed_expressions.iter() {
						println!("By equivalence [{}] we get: {}", equivalence, transformed);
					}
				}
			},
			Err(e) => panic!("err: {}", e),
		}
	}
}
