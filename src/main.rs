mod parser;
mod tree_transform;
mod transformation_graph;

use std::io;
use std::rc::Rc;

fn main() {
	loop {
		println!("Enter a mathematical expression");
		let mut expr = String::new();

		io::stdin()
			.read_line(&mut expr)
			.expect("Failed to read expression");

		let e = Rc::new(parser::parse(expr.trim()).unwrap());
		println!("Parsed expression: {}", e);
		// The graph takes ownership and persists `e`, but i can't figure out how to tell that to the compiler,
		// so we need to clone it.
		let mut graph = transformation_graph::create_graph(Rc::clone(&e));
		let equivalences = tree_transform::get_transformations();
		// Ensure that all expressions have long lifetime, so their references in the graph are safe.
		for equivalence in equivalences.iter() {
			for transformed in tree_transform::transform(e.as_ref(), equivalence).into_iter() {
				println!("By equivalence [{}] we get: {}", equivalence, transformed);
				graph.add_node(Rc::clone(&e), Rc::new(transformed), equivalence);
			}
		}
		println!("Graph:\n{}", graph)
	}
}
