//! The Ember-lox Abstract Syntax Tree (AST) module.

use ember_lox_rt::prelude::*;
use std::{fmt::Display, sync::Arc};

pub type Str = Arc<str>;

pub mod prelude {
  pub use super::{Expr, LiteralValue, Operator, Stmt, Str};
}

/// Box<Stmt> => prevent recursive definition (infinite size)
#[derive(Debug, Clone)]
pub enum Stmt {
  Block(Vec<Stmt>),
  Expression(Expr),
  Print(Expr),
  Var(Str, Option<Expr>),
  If(Expr, Box<Stmt>, Option<Box<Stmt>>),
  While(Expr, Box<Stmt>),
  Function(Str, Vec<Str>, Vec<Stmt>),
  Return(Option<Expr>),
}

/// Box<Expr> => prevent recursive definition (infinite size)
#[derive(Debug, Clone)]
pub enum Expr {
  Assign {
    name: Str,
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
    name: Str,
  },
  Grouping(Box<Expr>),
  Literal(LiteralValue),
  Logical {
    left: Box<Expr>,
    op: Operator,
    right: Box<Expr>,
  },
  Set {
    obj: Box<Expr>,
    name: Str,
    val: Box<Expr>,
  },
  Super {
    keyword: Str,
    method: Str,
  },
  This(Str),
  Unary {
    op: Operator,
    right: Box<Expr>,
  },
  Var(Str),
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
  String(Str),
  Bool(bool),
  Nil,
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
