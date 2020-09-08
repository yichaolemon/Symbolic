use std::fmt;
use regex::Regex;


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

// Grammar:
// Assign ← id = Sums
// Sums ← Sums + Products
// Sums ← Products
// Products ← Products * Value
// Products ← Value
// Value ← int
// Value ← id

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
      Ok((Expression::Variable(expr.to_string()), ""))
    }
  }
}

fn parse_literal(mut expr: &str) -> ParseResult {
  let multiplier = if expr.starts_with("-") {
    expr = expr.get(1..).unwrap();
    -1.0
  } else {
    1.0
  };
  let mut num = 0.0;
  while !expr.is_empty() {
    let first_char = expr.chars().next().unwrap();
    if first_char < '0' || first_char > '9' {
      break;
    }
		num *= 10.0;
    num += (first_char as u32 - '0' as u32) as f64;
    expr = expr.get(1..).unwrap();
  }
	// TODO: handle decimal points
  Ok((Expression::Constant(multiplier * num), expr))
}


fn parse_sum(expr: &str) -> ParseResult {
  let (mut s1, mut leftover) = parse_product(expr)?;
  while leftover.starts_with("+") {
		let (s2, leftover2) = parse_product(leftover.get(1..).unwrap())?;
    s1 = Expression::Sum(Box::new(s1), Box::new(s2));
    leftover = leftover2;
  }
  Ok((s1, leftover))
}

// TODO: consider having all of these functions take in a &str. is that more efficient?
fn parse_product(expr: &str) -> ParseResult {
  let (mut s1, mut leftover) = parse_leaf(expr)?;
  while leftover.starts_with("*") {
    let (s2, leftover2) = parse_leaf(leftover.get(1..).unwrap())?;
    s1 = Expression::Product(Box::new(s1), Box::new(s2));
    leftover = leftover2;
  }
  Ok((s1, leftover))
}

