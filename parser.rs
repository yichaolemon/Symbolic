use std::fmt;

#[derive(Debug)]
pub enum Expression {
  Sum(Box<Expression>, Box<Expression>),
  Product(Box<Expression>, Box<Expression>),
  Constant(f64),
  Variable(String),
}

impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Expression::Sum(e1, e2) => write!(f, "({})+({})", e1, e2),
      Expression::Product(e1, e2) => write!(f, "({})*({})", e1, e2),
      Expression::Constant(c) => write!(f, "{}", c),
      Expression::Variable(v) => write!(f, "{}", v),
    }
  }
}

#[derive(Debug)]
pub struct ParseError {
  input: String,
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Parse Error for {}", self.input)
  }
}

pub fn parse(expr: String) -> Result<Expression, ParseError> {
  println!("parser called");
  Err(ParseError {input: expr} )
  // Ok(Expression::Constant(0.0))
}

fn parse_subexpr(expr: String, level: u32) -> Result<Expression, ParseError> {
  Err(ParseError{input: expr})
}

