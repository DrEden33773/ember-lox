use crate::visit::{Visitor, VisitorAcceptor};
use ember_lox_rt::prelude::*;
use std::fmt::Display;

/// Box<Expr> => prevent recursive definition (infinite size)
#[derive(Debug, Clone)]
pub enum Expr {
  Assign {
    name: STR,
    val: Box<Expr>,
  },
  Binary {
    left: Box<Expr>,
    op: Operator,
    right: Box<Expr>,
  },
  Call {
    callee: Box<Expr>,
    args: Vec<Expr>,
  },
  Get {
    obj: Box<Expr>,
    name: STR,
  },
  Grouping {
    expr: Box<Expr>,
  },
  Literal {
    val: LiteralValue,
  },
  Logical {
    left: Box<Expr>,
    op: Operator,
    right: Box<Expr>,
  },
  Set {
    obj: Box<Expr>,
    name: STR,
    val: Box<Expr>,
  },
  Super {
    keyword: STR,
    method: STR,
  },
  This {
    keyword: STR,
  },
  Unary {
    op: Operator,
    right: Box<Expr>,
  },
  Var {
    name: STR,
  },
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
