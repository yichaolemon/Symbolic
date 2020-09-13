mod parser;
mod tree_transform;
mod transformation_graph;
mod measure;

use std::io;
use std::rc::Rc;
use std::collections::VecDeque;

// completely arbitrary
fn max_measure(min_measure: i32) -> i32 {
	return min_measure * 2 + 3;
}

fn main() {
	loop {
		println!("Enter a mathematical expression");
		let mut expr = String::new();

		io::stdin()
			.read_line(&mut expr)
			.expect("Failed to read expression");

		let root_exp = Rc::new(parser::parse(expr.trim()).unwrap());
		let mut min_exp_measure = measure::measure(root_exp.as_ref());
		let mut min_exp = Rc::clone(&root_exp);
		let mut min_exp_depth = 0;
		println!("Parsed expression: {}", root_exp);
		// The graph takes ownership and persists `e`, but i can't figure out how to tell that to the compiler,
		// so we need to clone it.
		let mut graph = transformation_graph::create_graph(Rc::clone(&root_exp));
		let equivalences = tree_transform::get_transformations();
		let mut to_transform = VecDeque::new();
		to_transform.push_back((Rc::clone(&root_exp), 0));
		let mut prev_depth = 0;
		while !to_transform.is_empty() {
			let (e, depth) = to_transform.pop_front().unwrap();
			if depth > prev_depth {
				println!("Reached depth {} of transformations", depth);
				prev_depth = depth;
			}
			for equivalence in equivalences.iter() {
				for transformed in tree_transform::transform(e.as_ref(), equivalence).into_iter() {
					// measure transformed to make sure it does not stray too far from root_exp
					let transformed_measure = measure::measure(&transformed);
					let transformed = Rc::new(transformed);
					if transformed_measure >= max_measure(min_exp_measure) { continue; }
					if transformed_measure < min_exp_measure {
						min_exp_measure = transformed_measure;
						min_exp = Rc::clone(&transformed);
						min_exp_depth = depth+1;
					}
					// println!("By equivalence [{}] we get: {}", equivalence, transformed);
					if graph.add_node(Rc::clone(&e), Rc::clone(&transformed), equivalence) {
						to_transform.push_back((transformed, depth+1));
					}
				}
			}
		}
		println!("Graph:\n{}", graph);
		println!("Min Exp: {} with measure {} is distance {} away from {}", min_exp, min_exp_measure, min_exp_depth, root_exp);
	}
}
