use crate::parser::Expression;
use std::borrow::Borrow;

/// wants a data structure that encompasses code transformation, before -> after
pub struct Equivalence {
	before: Expression,
	after: Expression,
}

pub fn get_transformations() -> [Equivalence; 4] {
	let a = Expression::Variable("a".into_string());
	let b = Expression::Variable("b".into_string());
	let c = Expression::Variable("c".into_string());
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