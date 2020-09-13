use crate::parser::Expression;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::Deref;
use std::fmt;

/// wants a data structure that encompasses code transformation, before -> after
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Equivalence {
	before: Expression,
	after: Expression,
}

impl fmt::Display for Equivalence {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} = {}", self.before, self.after)
	}
}

// ((0)*(4))+((1)*(0))

pub fn get_transformations() -> Vec<Equivalence> {
	let a = Expression::Variable("a".into());
	let b = Expression::Variable("b".into());
	let c = Expression::Variable("c".into());
	let zero = Expression::Constant(0);
	let one = Expression::Constant(1);
	let neg_one = Expression::Constant(-1);
	vec![
		// distributive
		Equivalence {
			before: a.clone() * (b.clone() + c.clone()),
			after: a.clone() * b.clone() + a.clone() * c.clone()
		},
		// commutative
		Equivalence {
			before: a.clone() + b.clone(),
			after: b.clone() + a.clone()
		},
		Equivalence {
			before: a.clone() * b.clone(),
			after: b.clone() * a.clone()
		},
		// associative
		Equivalence {
			before: a.clone() * (b.clone() * c.clone()),
			after: (a.clone() * b.clone()) * c.clone()
		},
		Equivalence {
			before: a.clone() + (b.clone() + c.clone()),
			after: (a.clone() + b.clone()) + c.clone()
		},
		// identity
		Equivalence {
			before: one.clone() * a.clone(),
			after: a.clone()
		},
		Equivalence {
			before: zero.clone() + a.clone(),
			after: a.clone(),
		},
		// inverse
		Equivalence {
			before: a.clone() + neg_one * a.clone(),
			after: zero.clone()
		},
		// Equivalence {
		// 	before: a.clone() * zero.clone(),
		// 	after: a.clone()
		// }
	]
}

pub fn match_expression_variables<'a>(exp: &'a Expression, match_exp: &Expression, assignments: &mut HashMap<String, &'a Expression>)
																	-> bool {
	// exp = 1 + 2, match_exp = a + b, etc
	match match_exp {
		Expression::Variable(s) => { assignments.insert(s.clone(), exp); true},
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
			}
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
			Expression::Product(apply_transform(c, assignments).into(), apply_transform(d, assignments).into())
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
	// TODO: filter out duplicates
	match transform_full_tree(exp, &equiv.after, &equiv.before) {
		Some(e) => transformed.push(e),
		None => ()
	}

	match exp {
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
		_ => ()
	}

	transformed
}
