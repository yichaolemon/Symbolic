use crate::{var, c, parser::{Expression, expression}};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::Deref;
use std::fmt;

/// wants a data structure that encompasses code transformation, before -> after
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Equivalence {
	before: Expression,
	after: Expression,
	forwards_only: bool,
}

impl fmt::Display for Equivalence {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} = {}", self.before, self.after)
	}
}

// example:
// x*0
// x*(a+(-1)*a)
// x*a + x*((-1)*a)
// x*a + (x*(-1))*a
// x*a + (-1)*(x*a)
// 0


pub fn get_transformations() -> Vec<Equivalence> {
	vec![
		// distributive
		Equivalence {
			before: expression("a*(b+c)"),
			after: expression("a*b+a*c"),
			forwards_only: false,
		},
		// commutative
		Equivalence {
			before: expression("a+b"),
			after: expression("b+a"),
			forwards_only: false,
		},
		Equivalence {
			before: expression("a*b"),
			after: expression("b*a"),
			forwards_only: false,
		},
		// associative
		Equivalence {
			before: expression("a*(b*c)"),
			after: expression("(a*b)*c"),
			forwards_only: false,
		},
		Equivalence {
			before: expression("a+(b+c)"),
			after: expression("(a+b)+c"),
			forwards_only: false,
		},
		// identity
		Equivalence {
			before: expression("1*a"),
			after: var!("a"),
			forwards_only: true,
		},
		Equivalence {
			before: expression("0+a"),
			after: var!("a"),
			forwards_only: true,
		},
		// inverse
		Equivalence {
			before: expression("a-a"),
			after: c!(0),
			forwards_only: true,
		},
		// complex ops
		Equivalence {
			before: expression("a/a"),
			after: c!(1),
			forwards_only: true,
		},
		Equivalence {
			before: expression("a/b"),
			after: expression("a*b^(-1)"),
			forwards_only: false,
		},
		Equivalence {
			before: expression("a-b"),
			after: expression("a+(-1)*b"),
			forwards_only: false,
		},
		Equivalence {
			before: expression("a^2"),
			after: expression("a*a"),
			forwards_only: false,
		},
		// misc simple
		Equivalence {
			before: expression("a*0"),
			after: c!(0),
			forwards_only: true,
		},
		Equivalence {
			before: expression("2*a"),
			after: expression("a+a"),
			forwards_only: false,
		},
	]
}

// Returns true if exp matches the pattern of match_exp.
pub fn match_expression_variables<'a>(exp: &'a Expression, match_exp: &Expression, assignments: &mut HashMap<String, &'a Expression>)
																	-> bool {
	// e.g. exp = 1 + 2, match_exp = a + b
	match match_exp {
		Expression::Variable(s) => match assignments.get(s) {
			Some(assignment) => (**assignment) == *exp,
			None => {assignments.insert(s.clone(), exp); true}
		},
		Expression::Constant(c) => match exp { Expression::Constant(d) => c == d, _ => false},
		Expression::Sum(a, b) =>
			match exp {
				Expression::Sum(c, d) =>
					match_expression_variables(c, a, assignments) && match_expression_variables(d, b, assignments),
				_ => false
			},
		Expression::Product(a, b) =>
			match exp {
				Expression::Product(c, d) =>
					match_expression_variables(c, a, assignments) && match_expression_variables(d, b, assignments),
				_ => false
			},
		Expression::Difference(a, b) =>
			match exp {
				Expression::Difference(c, d) =>
					match_expression_variables(c, a, assignments) && match_expression_variables(d, b, assignments),
				_ => false
			},
		Expression::Quotient(a, b) =>
			match exp {
				Expression::Quotient(c, d) =>
					match_expression_variables(c, a, assignments) && match_expression_variables(d, b, assignments),
				_ => false
			},
		Expression::Power(a, b) =>
			match exp {
				Expression::Power(c, d) =>
					match_expression_variables(c, a, assignments) && match_expression_variables(d, b, assignments),
				_ => false
			},
	}
}

fn apply_transform(match_exp: &Expression, assignments: &HashMap<String, &Expression>) -> Expression {
	match match_exp {
		Expression::Variable(s) => match assignments.get(s) {
			Some(exp) => (**exp).clone(),
			None => match_exp.clone() // this should not happen, if we call `match_expression_variables` first
		},
		Expression::Constant(_) => match_exp.clone(),
		Expression::Sum(a, b) =>
			Expression::Sum(apply_transform(a, assignments).into(), apply_transform(b, assignments).into()),
		Expression::Product(c, d) =>
			Expression::Product(apply_transform(c, assignments).into(), apply_transform(d, assignments).into()),
		Expression::Difference(c, d) =>
			Expression::Difference(apply_transform(c, assignments).into(), apply_transform(d, assignments).into()),
		Expression::Quotient(c, d) =>
			Expression::Quotient(apply_transform(c, assignments).into(), apply_transform(d, assignments).into()),
		Expression::Power(c, d) =>
			Expression::Power(apply_transform(c, assignments).into(), apply_transform(d, assignments).into()),
	}
}

fn transform_full_tree(exp: &Expression, before: &Expression, after: &Expression) -> Option<Expression> {
	let mut assignments = HashMap::new();
	if !match_expression_variables(exp, before, assignments.borrow_mut())
	{ return None }
	Some(apply_transform(after, &assignments))
}

/// Given an expression, and an equivalence, outputs a list of expressions equivalent to it,
/// that can be reached by applying the equivalence once.
pub fn transform(exp: &Expression, equiv: &Equivalence) -> Vec<Expression> {
	let mut transformed = Vec::new();

	match transform_full_tree(exp, &equiv.before, &equiv.after) {
		Some(e) => transformed.push(e),
		None => ()
	}
	if !equiv.forwards_only {
		match transform_full_tree(exp, &equiv.after, &equiv.before) {
			Some(e) => transformed.push(e),
			None => ()
		}
	}

	match exp {
		Expression::Constant(_) => (),
		Expression::Variable(_) => (),
		Expression::Product(a, b) => {
			for e in transform(a, equiv).into_iter() {
				transformed.push(e * b.deref().clone())
			}
			for e in transform(b, equiv).into_iter() {
				transformed.push(a.deref().clone() * e)
			}
		},
		Expression::Sum(a, b) => {
			for e in transform(a, equiv).into_iter() {
				transformed.push(e + b.deref().clone())
			}
			for e in transform(b, equiv).into_iter() {
				transformed.push(a.deref().clone() + e)
			}
		},
		Expression::Difference(a, b) => {
			for e in transform(a, equiv).into_iter() {
				transformed.push(e - b.deref().clone())
			}
			for e in transform(b, equiv).into_iter() {
				transformed.push(a.deref().clone() - e)
			}
		},
		Expression::Quotient(a, b) => {
			for e in transform(a, equiv).into_iter() {
				transformed.push(e / b.deref().clone())
			}
			for e in transform(b, equiv).into_iter() {
				transformed.push(a.deref().clone() / e)
			}
		},
		Expression::Power(a, b) => {
			for e in transform(a, equiv).into_iter() {
				transformed.push(e ^ b.deref().clone())
			}
			for e in transform(b, equiv).into_iter() {
				transformed.push(a.deref().clone() ^ e)
			}
		},
	}

	transformed
}
