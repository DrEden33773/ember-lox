use crate::visit::{Visitor, VisitorAcceptor};
use ember_lox_rt::prelude::*;
use ember_lox_tokenizer::TokenKind;
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
      LiteralValue::Number(n) => {
        // At lease should have one decimal (fraction) place
        let mut output = n.to_string();
        if !output.contains('.') {
          output.push_str(".0");
        }
        write!(f, "{}", output)
      }
      LiteralValue::String(s) => {
        // Tokenizer will gather `"` at the beginning and ending,
        // but in the test case, we don't need to print them.
        debug_assert!(s.starts_with('"') && s.ends_with('"'));
        write!(f, "{}", &s[1..s.len() - 1])
      }
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
      Not => "!",
    };
    f.write_str(op_str)
  }
}
