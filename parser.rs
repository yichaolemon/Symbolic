use std::fmt;
pub enum Expression {
  Sum(Box<Expression>, Box<Expression>),
  Product(Box<Expression>, Box<Expression>),
  Constant(f64),
  Variable(String),
}

pub struct ParseError {
  input: String,
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Parse Error")
  }
}

pub fn parse(expr: String) -> Result<Expression, ParseError> {
  println!("parser called");
  Err(ParseError {input: expr} )
  // Ok(Expression::Constant(0.0))
}
