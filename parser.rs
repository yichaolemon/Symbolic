pub enum Expression {
  Sum(Box<Expression>, Box<Expression>),
  Product(Box<Expression>, Box<Expression>),
  Constant(f64),
  Variable(String),
}

pub struct ParseError {
  input: String,
}

pub fn parse(expr: String) -> Result<Expression, ParseError> {
  println!("parser called");
  Ok(Expression::Constant(0.0))
}
