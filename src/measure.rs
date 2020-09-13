use crate::{var, c, parser::{Expression, parse, ParseError}};
use crate::{transformation_graph, tree_transform};
use std::rc::Rc;
use std::collections::VecDeque;

const MEASURE_PER_HEIGHT: i32 = 1;
const VARIABLE_CONST: i32 = 2;
const CONSTANT_CONST: i32 = 1;

pub fn measure(e: &Expression) -> i32 {
	match e {
		Expression::Constant(_) => CONSTANT_CONST,
		Expression::Variable(_) => VARIABLE_CONST,
		Expression::Sum(a, b) => measure(a) + measure(b) + MEASURE_PER_HEIGHT,
		Expression::Product(a, b) => measure(a) + measure(b) + MEASURE_PER_HEIGHT,
		Expression::Difference(a, b) => measure(a) + measure(b) + MEASURE_PER_HEIGHT,
		Expression::Quotient(a, b) => measure(a) + measure(b) + MEASURE_PER_HEIGHT,
		Expression::Power(a, b) => measure(a) + measure(b) + MEASURE_PER_HEIGHT,
	}
}

// completely arbitrary
fn max_measure(min_measure: i32) -> i32 {
	return min_measure * 2 + 3;
}

pub fn find_min_equivalent_expr(e: Expression) -> Expression {
	let root_exp = Rc::new(e);
	let mut min_exp_measure = measure(root_exp.as_ref());
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
				let transformed_measure = measure(&transformed);
				let transformed = Rc::new(transformed);
				if transformed_measure >= max_measure(min_exp_measure) { continue; }
				if transformed_measure < min_exp_measure {
					min_exp_measure = transformed_measure;
					min_exp = Rc::clone(&transformed);
					min_exp_depth = depth+1;
				}
				if graph.add_node(Rc::clone(&e), Rc::clone(&transformed), equivalence) {
					to_transform.push_back((transformed, depth+1));
				}
			}
		}
	}
	println!("Graph:\n{}", graph);
	println!("{} with measure {} is distance {} away from {}", min_exp, min_exp_measure, min_exp_depth, root_exp);
	min_exp.as_ref().clone()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_multiply_by_zero_expression() {
		// Test with expression constructors.
		assert_eq!(find_min_equivalent_expr(var!("x") * c!(0)), c!(0));
	}

	fn assert_min_equivalent(e: &str, min: &str) -> Result<(), ParseError> {
		let e = parse(e)?;
		let min = parse(min)?;
		assert_eq!(find_min_equivalent_expr(e), min);
		Ok(())
	}

	#[test]
	fn test_multiply_by_zero() -> Result<(), ParseError> {
		// Test with parsing.
		assert_min_equivalent("x*0", "0")
	}

	#[test]
	fn test_multiply_by_two() -> Result<(), ParseError> {
		assert_min_equivalent("a*b+a*b", "2*(a*b)")
	}

	#[test]
	fn test_division_cancel() -> Result<(), ParseError> {
		assert_min_equivalent("(a*b)/a", "b")
	}

	#[test]
	fn test_expansion() -> Result<(), ParseError> {
		assert_min_equivalent("(a+b)*(a-b)", "a^2-b^2")
	}

	#[test]
	fn test_factoring() -> Result<(), ParseError> {
		assert_min_equivalent("(a^2+a*b+a*b+b^2)/(a+b)", "a+b")
	}
}
