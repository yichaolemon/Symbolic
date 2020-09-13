use crate::parser::Expression;

const MEASURE_PER_HEIGHT: i32 = 1;
const VARIABLE_CONST: i32 = 1;
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