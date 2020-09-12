use crate::parser::Expression;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::Deref;
use std::fmt;

/// wants a data structure that encompasses code transformation, before -> after
pub struct Equivalence {
	before: Expression,
	after: Expression,
}

impl fmt::Display for Equivalence {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} = {}", self.before, self.after)
	}
}

pub fn get_transformations() -> [Equivalence; 4] {
	let a = Expression::Variable("a".into());
	let b = Expression::Variable("b".into());
	let c = Expression::Variable("c".into());
	[
		Equivalence {
			before: a.clone() * (b.clone() + c.clone()),
			after: a.clone() * b.clone() + a.clone() * c.clone()
		},
		Equivalence {
			before: a.clone() + b.clone(),
			after: b.clone() + a.clone()
		},
		Equivalence {
			before: a.clone() * b.clone(),
			after: b.clone() * a.clone()
		},
		Equivalence {
			before: a.clone() * (b.clone() * c.clone()),
			after: (a.clone() * b.clone()) * c.clone()
		},
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

pub fn apply_transform(match_exp: &Expression, assignments: &HashMap<String, &Expression>) -> Expression {
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

pub fn transform_full_tree(exp: &Expression, equiv: &Equivalence) -> Option<Expression> {
	let mut assignments = HashMap::new();
	if !match_expression_variables(exp, &equiv.before, assignments.borrow_mut())
	{ return None }
	Some(apply_transform(&equiv.after, &assignments))
}

pub fn transform_rec<F: FnMut(Expression)>(exp: &Expression, equiv: &Equivalence, mut add_transformed: F) {
	match transform_full_tree(exp, equiv) {
		Some(e) => add_transformed(e),
		None => ()
	}

	match exp {
		Expression::Product(a, b) => {
			transform_rec(a, equiv, |e| add_transformed(e * b.deref().clone()));
			transform_rec(b, equiv, |e| add_transformed(a.deref().clone() * e));
		},
		Expression::Sum(a, b) => {
			transform_rec(a, equiv, |e| add_transformed(e + b.deref().clone()));
			transform_rec(b, equiv, |e| add_transformed(a.deref().clone() + e));
		},
		_ => ()
	}
}

/// Given an expression, and an equivalence, outputs a list of expressions equivalent to it,
/// that can be reached by applying the equivalence once.
pub fn transform(exp: &Expression, equiv: &Equivalence) -> Vec<Expression> {
	let mut transformed = Vec::new();
	transform_rec(exp, equiv, |e| transformed.push(e));
	transformed
}

