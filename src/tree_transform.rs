use crate::{var, c, parser::{Expression, expression}};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::Deref;
use std::fmt;

type EquivMethod = Box<dyn Fn(&Expression) -> Option<Expression>>;

/// wants a data structure that encompasses code transformation, before -> after
#[derive(Default)]
pub struct Equivalence {
	before: Expression,
	after: Expression,
	forwards_only: bool,
	method: Option<EquivMethod>,
	method_name: String,
}

impl fmt::Display for Equivalence {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.method {
			Some(_) => write!(f, "{}", self.method_name),
			None => write!(f, "{} = {}", self.before, self.after),
		}
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
			..Default::default()
		},
		Equivalence {
			before: expression("(a*b)^c"),
			after: expression("a^c*b^c"),
			..Default::default()
		},
		// commutative
		Equivalence {
			before: expression("a+b"),
			after: expression("b+a"),
			..Default::default()
		},
		Equivalence {
			before: expression("a*b"),
			after: expression("b*a"),
			..Default::default()
		},
		// associative
		Equivalence {
			before: expression("a*(b*c)"),
			after: expression("(a*b)*c"),
			..Default::default()
		},
		Equivalence {
			before: expression("a+(b+c)"),
			after: expression("(a+b)+c"),
			..Default::default()
		},
		// identity
		// multiplicative identity handled by split_repeated_operation
		Equivalence {
			before: expression("0+a"),
			after: var!("a"),
			forwards_only: true,
			..Default::default()
		},
		// inverse
		Equivalence {
			before: expression("a-a"),
			after: c!(0),
			forwards_only: true,
			..Default::default()
		},
		// complex ops
		Equivalence {
			before: expression("a/b"),
			after: expression("a*b^(-1)"),
			..Default::default()
		},
		Equivalence {
			before: expression("a^b*a^c"),
			after: expression("a^(b+c)"),
			..Default::default()
		},
		Equivalence {
			before: expression("a^b^c"),
			after: expression("a^(b*c)"),
			..Default::default()
		},
		Equivalence {
			before: expression("a-b"),
			after: expression("a+(-1)*b"),
			..Default::default()
		},
		// misc simple
		Equivalence {
			before: expression("a*0"),
			after: c!(0),
			forwards_only: true,
			..Default::default()
		},
		Equivalence {
			before: expression("a^1"),
			after: expression("a"),
			forwards_only: true,
			..Default::default()
		},
		// simplify expressions with only constants by evaluation
		Equivalence {
			method: Some(Box::new(move |exp| exp.eval_const())),
			method_name: "eval_const".into(),
			..Default::default()
		},
		Equivalence {
			method: Some(Box::new(move |exp| multiplicative_inverse(exp))),
			method_name: "multiplicative_inverse".into(),
			..Default::default()
		}
		// Equivalence {
		// 	method: Some(Box::new(move |exp| split_repeated_operation(exp))),
		// 	method_name: "split_repeated_op".into(),
		// 	..Default::default()
		// },
	]
}

// a^0 is 1, and a/a is 1, unless a is a constant zero
fn multiplicative_inverse(exp: &Expression) -> Option<Expression> {
	match exp {
		Expression::Power(a, b) => {
			if b.unwrap_constant()? == 0 && a.eval_const() != Some(Expression::Constant(0)) {
				return Some(c!(1))
			}
		}
		Expression::Quotient(a, b) => {
			if a == b && a.eval_const() != Some(Expression::Constant(0)) {
				return Some(c!(1))
			}
		}
		_ => ()
	};
	None
}

// // Turn a*a into a^2
// fn group_repeated_operation(exp: &Expression) -> Option<Expression> {
// 	match exp {
// 		// 2*a = a+a
// 		// 6*a = a+5*a
// 		Expression::Product(a, b) => {
// 			let c = a.unwrap_constant()?;
// 			if c == 0 { Some(c!(0)) }
// 			else if c < -1 {
// 				Some(c!(-1) * b.deref().clone() + c!(c+1) * b.deref().clone())
// 			} else if c == 1 {
// 				Some(b.deref().clone())
// 			} else if c > 1 {
// 				Some(b.deref().clone() + c!(c-1) * b.deref().clone())
// 			}
// 			else { None }
// 		},
//
// 		// a^2 = a*a
// 		// a^3 = a^2*a
// 		Expression::Power(a, b) => {
// 			let d = b.unwrap_constant()?;
// 			if d == 0 { Some(c!(1)) }
// 			else if d < -1 {
// 				Some(a.deref().clone() ^ c!(-1) * a.deref().clone() ^ c!(c+1))
// 			} else if d == 1 {
// 				Some(a.deref().clone())
// 			} else if d > 1 {
// 				Some(a.deref().clone * a.deref().clone() ^ c!(c-1))
// 			}
// 			else { None }
// 		},
//
// 		_ => None
// 	}
// }

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

	match equiv.method.as_ref() {
		Some(m) => match m(exp) {
			Some(e) => transformed.push(e),
			None => ()
		}
		None => {
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
