use std::fmt;
use regex::Regex;
use std::ops::{Mul, Add, Sub, Div, BitXor};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
  Constant(i32),
  Variable(String),
  Sum(Box<Expression>, Box<Expression>),
  Product(Box<Expression>, Box<Expression>),
  Difference(Box<Expression>, Box<Expression>),
  Quotient(Box<Expression>, Box<Expression>),
  Power(Box<Expression>, Box<Expression>),
}

impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Expression::Constant(c) => write!(f, "{}", c),
      Expression::Variable(v) => write!(f, "{}", v),
      Expression::Sum(e1, e2) => write!(f, "({})+({})", e1, e2),
      Expression::Product(e1, e2) => write!(f, "({})*({})", e1, e2),
      Expression::Difference(e1, e2) => write!(f, "({})-({})", e1, e2),
      Expression::Quotient(e1, e2) => write!(f, "({})/({})", e1, e2),
      Expression::Power(e1, e2) => write!(f, "({})^({})", e1, e2),
    }
  }
}

impl Mul<Expression> for Expression {
  type Output = Expression;

  fn mul(self, rhs: Expression) -> Self::Output {
    Expression::Product(self.clone().into(), rhs.clone().into())
  }
}

impl Add<Expression> for Expression {
  type Output = Expression;

  fn add(self, rhs: Expression) -> Self::Output {
    Expression::Sum(self.into(), rhs.into())
  }
}

impl Sub<Expression> for Expression {
  type Output = Expression;

  fn sub(self, rhs: Expression) -> Self::Output {
    Expression::Difference(self.into(), rhs.into())
  }
}

impl Div<Expression> for Expression {
  type Output = Expression;

  fn div(self, rhs: Expression) -> Self::Output {
    Expression::Quotient(self.into(), rhs.into())
  }
}

// abuse notation. it deserves it.
impl BitXor<Expression> for Expression {
  type Output = Expression;

  fn bitxor(self, rhs: Expression) -> Self::Output {
    Expression::Power(self.into(), rhs.into())
  }
}

// usage: var!("a")
#[macro_export]
macro_rules! var {
  ($s:expr) => { Expression::Variable(($s).into()) }
}

// usage: c!(1)
#[macro_export]
macro_rules! c {
  ($c:expr) => { Expression::Constant($c) }
}

#[derive(Debug)]
pub struct ParseError {
  msg: String,
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Parse Error: {}", self.msg)
  }
}

pub fn parse(expr: &str) -> Result<Expression, ParseError> {
	let (expr, leftover) = parse_sum(expr)?;
  if leftover.is_empty() {
    Ok(expr)
  } else {
    Err(ParseError{msg: format!("expected end of expression. remaining to parse: '{}'", leftover)})
  }
}

// TODO: shift-reduce parser

type ParseResult<'a> = Result<(Expression, &'a str), ParseError>;

// can be used at lowest (leaf) level of parse tree
// i.e. it's a number, a variable, or a subexpression in parentheses
fn parse_leaf(expr: &str) -> ParseResult {
	if expr.starts_with("(") {
    // Surprise! The leaf is a subexpression in parentheses. So we have to keep parsing.
    let (s1, leftover) = parse_sum(expr.get(1..).unwrap())?;
    if leftover.starts_with(")") {
      Ok((s1, leftover.get(1..).unwrap()))
    } else {
      Err(ParseError{msg: String::from("missing end parenthesis")})
    }
  } else {
    let number_regex = Regex::new(r"^[-0-9]").unwrap();
    if number_regex.is_match(expr) {
      parse_literal(expr)
    } else {
      parse_variable(expr)
    }
  }
}

fn parse_variable(mut expr: &str) -> ParseResult {
  let mut curr_str = String::new();
  while !expr.is_empty() {
    if Regex::new(r"^[\w]").unwrap().is_match(expr) {
      curr_str.push(expr.chars().next().unwrap());
      expr = expr.get(1..).unwrap();
    } else {
      break;
    }
  }
  Ok((Expression::Variable(curr_str), expr))
}

fn parse_literal(mut expr: &str) -> ParseResult {
  let multiplier = if expr.starts_with("-") {
    expr = expr.get(1..).unwrap();
    -1
  } else { 1 };

  let mut num = 0;
  while !expr.is_empty() {
    let first_char = expr.chars().next().unwrap();
    if first_char < '0' || first_char > '9' {
      break;
    }
		num *= 10;
    num += first_char as i32 - '0' as i32;
    expr = expr.get(1..).unwrap();
  }
  Ok((Expression::Constant(multiplier * num), expr))
}

fn parse_sum(expr: &str) -> ParseResult {
  let (mut s1, mut leftover) = parse_product(expr)?;
  while leftover.starts_with("+") || leftover.starts_with("-") {
		let (s2, leftover2) = parse_product(leftover.get(1..).unwrap())?;
    if leftover.starts_with("+") {
      s1 = Expression::Sum(s1.into(), s2.into());
    } else {
      s1 = Expression::Difference(s1.into(), s2.into());
    }
    leftover = leftover2;
  }
  Ok((s1, leftover))
}

fn parse_product(expr: &str) -> ParseResult {
  let (mut s1, mut leftover) = parse_power(expr)?;
  while leftover.starts_with("*") || leftover.starts_with("/") {
    let (s2, leftover2) = parse_power(leftover.get(1..).unwrap())?;
    if leftover.starts_with("*") {
      s1 = Expression::Product(s1.into(), s2.into());
    } else {
      s1 = Expression::Quotient(s1.into(), s2.into());
    }
    leftover = leftover2;
  }
  Ok((s1, leftover))
}

fn parse_power(expr: &str) -> ParseResult {
  let (mut s1, mut leftover) = parse_leaf(expr)?;
  while leftover.starts_with("^") {
    let (s2, leftover2) = parse_leaf(leftover.get(1..).unwrap())?;
    s1 = Expression::Power(s1.into(), s2.into());
    leftover = leftover2;
  }
  Ok((s1, leftover))
}
