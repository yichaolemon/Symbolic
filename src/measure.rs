use crate::parser::Expression;

const SUM_CONST: i32 = 1;
const PRODUCT_CONST: i32 = 1;
const VARIABLE_CONST: i32 = 2;
const CONSTANT_CONST: i32 = 1;

pub fn measure(e: &Expression) -> i32 {
	match e {
		Expression::Constant(_) => CONSTANT_CONST,
		Expression::Variable(_) => VARIABLE_CONST,
		Expression::Sum(a, b) => measure(a) + measure(b) + SUM_CONST,
		Expression::Product(a, b) => measure(a) + measure(b) + PRODUCT_CONST,
	}
}