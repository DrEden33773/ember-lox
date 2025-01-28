use crate::{
  pool::prelude::*,
  visit::{Visitor, VisitorAcceptor},
  STR,
};
use ember_lox_tokenizer::TokenKind;
use std::{fmt::Display, sync::Arc};

#[derive(Debug, Clone)]
pub enum Expr {
  Assign {
    name: STR,
    val: Box<Expr>,
  },
  Binary {
    left: Box<Expr>,
    op: PosedOperator,
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
    val: PosedLiteral,
  },
  Logical {
    left: Box<Expr>,
    op: PosedOperator,
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
    op: PosedOperator,
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
pub struct PosedOperator(pub Operator, pub usize);

impl From<(Operator, usize)> for PosedOperator {
  fn from(value: (Operator, usize)) -> Self {
    Self(value.0, value.1)
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
  /// +
  Plus,
  /// -
  Minus,
  /// *
  Multiply,
  /// /
  Divide,
  /// ==
  Equal,
  /// !=
  NotEqual,
  /// >
  Greater,
  /// >=
  GreaterEqual,
  /// <
  Less,
  /// <=
  LessEqual,
  /// !
  Not,
}

impl TryFrom<TokenKind> for Operator {
  type Error = ();

  fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
    use TokenKind::*;
    match value {
      Plus => Ok(Operator::Plus),
      Minus => Ok(Operator::Minus),
      Star => Ok(Operator::Multiply),
      Slash => Ok(Operator::Divide),
      EqEq => Ok(Operator::Equal),
      BangEq => Ok(Operator::NotEqual),
      Gt => Ok(Operator::Greater),
      GtEq => Ok(Operator::GreaterEqual),
      Lt => Ok(Operator::Less),
      LtEq => Ok(Operator::LessEqual),
      Bang => Ok(Operator::Not),
      _ => Err(()),
    }
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
      Not => "!",
    };
    f.write_str(op_str)
  }
}

#[derive(Debug, Clone)]
pub struct PosedLiteral(pub LiteralValue, pub usize);

impl From<(LiteralValue, usize)> for PosedLiteral {
  fn from(value: (LiteralValue, usize)) -> Self {
    Self(value.0, value.1)
  }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
  Number(f64),
  String(Arc<str>),
  Bool(bool),
  Nil,
}

impl LiteralValue {
  pub fn is_true(&self) -> bool {
    match self {
      LiteralValue::Bool(b) => *b,
      LiteralValue::Nil => false,
      _ => true,
    }
  }

  pub fn get_type(&self) -> &str {
    match self {
      LiteralValue::Number(_) => "number",
      LiteralValue::String(_) => "string",
      LiteralValue::Bool(_) => "bool",
      LiteralValue::Nil => "nil",
    }
  }
}

impl std::cmp::PartialOrd for LiteralValue {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match (self, other) {
      (LiteralValue::Number(a), LiteralValue::Number(b)) => a.partial_cmp(b),
      _ => None,
    }
  }
}

impl std::cmp::PartialEq for LiteralValue {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (LiteralValue::Number(a), LiteralValue::Number(b)) => a == b,
      (LiteralValue::String(a), LiteralValue::String(b)) => a == b,
      (LiteralValue::Bool(a), LiteralValue::Bool(b)) => a == b,
      (LiteralValue::Nil, LiteralValue::Nil) => true,
      _ => false,
    }
  }
}

impl std::ops::Add for &LiteralValue {
  type Output = Result<LiteralValue, String>;

  fn add(self, rhs: &LiteralValue) -> Self::Output {
    match (self, rhs) {
      (LiteralValue::Number(a), LiteralValue::Number(b)) => Ok(LiteralValue::Number(a + b)),
      (LiteralValue::String(a), LiteralValue::String(b)) => {
        let new = a.to_string() + b.as_ref();
        Ok(new.as_str().into())
      }
      _ => Err(format!("Operands must be numbers.")),
    }
  }
}

impl std::ops::Sub for &LiteralValue {
  type Output = Result<LiteralValue, String>;

  fn sub(self, rhs: &LiteralValue) -> Self::Output {
    match (self, rhs) {
      (LiteralValue::Number(a), LiteralValue::Number(b)) => Ok(LiteralValue::Number(a - b)),
      _ => Err(format!("Operands must be numbers.")),
    }
  }
}

impl std::ops::Mul for &LiteralValue {
  type Output = Result<LiteralValue, String>;

  fn mul(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (LiteralValue::Number(a), LiteralValue::Number(b)) => Ok(LiteralValue::Number(a * b)),
      _ => Err(format!("Operands must be numbers.")),
    }
  }
}

impl std::ops::Div for &LiteralValue {
  type Output = Result<LiteralValue, String>;

  fn div(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (LiteralValue::Number(a), LiteralValue::Number(b)) => Ok(LiteralValue::Number(a / b)),
      _ => Err(format!("Operands must be numbers.")),
    }
  }
}

impl std::ops::Neg for &LiteralValue {
  type Output = Result<LiteralValue, String>;

  fn neg(self) -> Self::Output {
    match self {
      LiteralValue::Number(n) => Ok(LiteralValue::Number(-n)),
      _ => Err(format!("Operand must be a number.")),
    }
  }
}

impl std::ops::Not for &LiteralValue {
  type Output = Result<LiteralValue, String>;

  fn not(self) -> Self::Output {
    let res = self.is_true();
    Ok(LiteralValue::Bool(!res))
  }
}

impl Display for LiteralValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      LiteralValue::Number(n) => {
        // At lease should have one decimal (fraction) place
        let mut output = n.to_string();
        if !output.contains('.') {
          output.push_str(".0");
        }
        write!(f, "{}", output)
      }
      LiteralValue::String(s) => {
        // Has removed token's `"` at the beginning and end
        write!(f, "{}", s)
      }
      LiteralValue::Bool(b) => write!(f, "{}", b),
      LiteralValue::Nil => write!(f, "nil"),
    }
  }
}

impl From<Option<&str>> for LiteralValue {
  fn from(value: Option<&str>) -> Self {
    match value {
      Some(s) => LiteralValue::String(intern_string(s)),
      None => LiteralValue::Nil,
    }
  }
}
impl From<Option<f64>> for LiteralValue {
  fn from(value: Option<f64>) -> Self {
    match value {
      Some(n) => LiteralValue::Number(n),
      None => LiteralValue::Nil,
    }
  }
}
impl From<Option<bool>> for LiteralValue {
  fn from(value: Option<bool>) -> Self {
    match value {
      Some(b) => LiteralValue::Bool(b),
      None => LiteralValue::Nil,
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
