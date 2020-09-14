mod parser;
mod tree_transform;
mod transformation_graph;
mod measure;

use std::io;
use std::time::Instant;

fn main() {
	loop {
		println!("Enter a mathematical expression");
		let mut expr = String::new();

		io::stdin()
			.read_line(&mut expr)
			.expect("Failed to read expression");

		let now = Instant::now();
		let root_exp = parser::parse(expr.trim()).unwrap();
		measure::find_min_equivalent_expr(root_exp);
		println!("Elapsed time {}s", now.elapsed().as_secs());
	}
}
