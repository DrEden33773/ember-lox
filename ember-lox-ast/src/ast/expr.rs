use crate::visit::{Visitor, VisitorAcceptor};
use ember_lox_rt::prelude::*;
use std::fmt::Display;

/// Box<Expr> => prevent recursive definition (infinite size)
#[derive(Debug, Clone)]
pub enum Expr {
  Assign(Assign),
  Binary(Binary),
  Call(Call),
  Get(Get),
  Grouping(Grouping),
  Literal(Literal),
  Logical(Logical),
  Set(Set),
  Super(Super),
  This(This),
  Unary(Unary),
  Var(Var),
}

impl VisitorAcceptor for Expr {
  fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
    visitor.visit_expr(self)
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
  Plus,
  Minus,
  Multiply,
  Divide,
  Equal,
  NotEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
  Number(f64),
  String(STR),
  Bool(bool),
  Nil,
}

impl Display for LiteralValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      LiteralValue::Number(n) => write!(f, "{}", n),
      LiteralValue::String(s) => write!(f, "{}", s),
      LiteralValue::Bool(b) => write!(f, "{}", b),
      LiteralValue::Nil => write!(f, "nil"),
    }
  }
}

impl From<bool> for LiteralValue {
  fn from(value: bool) -> Self {
    LiteralValue::Bool(value)
  }
}
impl From<f64> for LiteralValue {
  fn from(value: f64) -> Self {
    LiteralValue::Number(value)
  }
}
impl From<&str> for LiteralValue {
  fn from(value: &str) -> Self {
    LiteralValue::String(intern_string(value))
  }
}

impl Display for Operator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use Operator::*;
    let op_str = match self {
      Plus => "+",
      Minus => "-",
      Multiply => "*",
      Divide => "/",
      Equal => "==",
      NotEqual => "!=",
      Greater => ">",
      GreaterEqual => ">=",
      Less => "<",
      LessEqual => "<=",
    };
    f.write_str(op_str)
  }
}

#[derive(Debug, Clone)]
pub struct Assign {
  pub name: STR,
  pub val: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Binary {
  pub left: Box<Expr>,
  pub op: Operator,
  pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Call {
  pub callee: Box<Expr>,
  pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Get {
  pub obj: Box<Expr>,
  pub name: STR,
}

#[derive(Debug, Clone)]
pub struct Grouping {
  pub expr: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Literal {
  pub val: LiteralValue,
}

#[derive(Debug, Clone)]
pub struct Logical {
  pub left: Box<Expr>,
  pub op: Operator,
  pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Set {
  pub obj: Box<Expr>,
  pub name: STR,
  pub val: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Super {
  pub keyword: STR,
  pub method: STR,
}

#[derive(Debug, Clone)]
pub struct This {
  pub keyword: STR,
}

#[derive(Debug, Clone)]
pub struct Unary {
  pub op: Operator,
  pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Var {
  pub name: STR,
}
