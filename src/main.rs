mod parser;
mod tree_transform;
mod transformation_graph;

use std::io;

fn main() {
	loop {
		println!("Enter a mathematical expression");
		let mut expr = String::new();

		io::stdin()
			.read_line(&mut expr)
			.expect("Failed to read expression");

		let e = parser::parse(expr.trim()).unwrap();
		println!("Parsed expression: {}", e);
		let mut graph = transformation_graph::create_graph(e);
		let equivalences = tree_transform::get_transformations();
		for equivalence in equivalences.iter() {
			let transformed_expressions = tree_transform::transform(&e, equivalence);
			for transformed in transformed_expressions.into_iter() {
				graph.add_node(&e, transformed, equivalence);
				println!("By equivalence [{}] we get: {}", equivalence, transformed);
			}
		}
		println!("Graph:\n{}", graph)
	}
}
