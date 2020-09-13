mod parser;
mod tree_transform;
mod transformation_graph;
mod measure;

use std::io;

fn main() {
	loop {
		println!("Enter a mathematical expression");
		let mut expr = String::new();

		io::stdin()
			.read_line(&mut expr)
			.expect("Failed to read expression");

		let root_exp = parser::parse(expr.trim()).unwrap();
		measure::find_min_equivalent_expr(root_exp);
	}
}
