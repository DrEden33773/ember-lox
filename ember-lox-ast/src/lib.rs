//! The Ember-lox Abstract Syntax Tree (AST) module.
//!
//! # Note
//!
//! This API is unstable.

pub mod ast;
pub mod pool;
pub mod visit;

use std::sync::Arc;

use crate::visit::{Visitor, VisitorAcceptor};
use ast::expr::Expr;
use ast::stmt::Stmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct STR(pub Arc<str>, pub usize);

impl From<(Arc<str>, usize)> for STR {
  fn from((s, line): (Arc<str>, usize)) -> Self {
    Self(s, line)
  }
}

impl From<(&str, usize)> for STR {
  fn from((s, line): (&str, usize)) -> Self {
    Self(s.into(), line)
  }
}

pub struct AstPrinter;

fn stringify_multi_lines(starting: &str, contents: &[String], ending: &str) -> String {
  let mut res = starting.to_string();
  let new_line = format!("\n{}", "".repeat(starting.len()));
  for (i, content) in contents.iter().enumerate() {
    match i {
      i if i == 0 => res += "::: ",
      i if i == contents.len() - 1 => res += " └─ ",
      _ => res += " ├─ ",
    }
    res += content;
    res += if i == contents.len() - 1 {
      ending
    } else {
      &new_line
    };
  }
  res
}

fn stringify_function(
  p: &mut AstPrinter,
  name: &STR,
  params: &Vec<STR>,
  body: &Vec<Stmt>,
) -> String {
  let params = params
    .iter()
    .map(|s| s.0.as_ref())
    .collect::<Vec<_>>()
    .join(", ");
  let body = body.iter().map(|s| s.accept(p)).collect::<Vec<_>>();
  let starting = if name.0.is_empty() {
    "(function ".to_string()
  } else {
    format!("(function {}({}) ", name.0, params)
  };
  stringify_multi_lines(&starting, &body, ")")
}

fn stringify_variable(p: &mut AstPrinter, name: &STR, initializer: &Option<Expr>) -> String {
  let init_str = initializer
    .as_ref()
    .map(|e| e.accept(p))
    .unwrap_or_default();
  format!("(var {} {})", name.0, init_str)
}

impl Visitor for AstPrinter {
  type Output = String;

  fn visit_stmt(&mut self, stmt: &Stmt) -> Self::Output {
    match stmt {
      Stmt::Block { stmts } => {
        let stmts = stmts.iter().map(|s| s.accept(self)).collect::<Vec<_>>();
        stringify_multi_lines("(block ", &stmts, ")")
      }
      Stmt::Class {
        name,
        superclass, // similar with Stmt::Variable
        methods,    // similar with Stmt::Function
      } => {
        let superclass = superclass
          .as_ref()
          .map(|(name, _)| name.0.to_string())
          .unwrap_or_default();
        let methods = methods
          .iter()
          .map(|(name, params, body)| stringify_function(self, name, params, body))
          .collect::<Vec<_>>();
        let starting = if superclass.is_empty() {
          format!("(class {} ", name.0)
        } else {
          format!("(class {} extends {} ", name.0, superclass)
        };
        stringify_multi_lines(&starting, &methods, ")")
      }
      Stmt::Expression { expr } => expr.accept(self),
      Stmt::Function { name, params, body } => stringify_function(self, name, params, body),
      Stmt::If {
        cond,
        then_branch,
        else_branch,
      } => {
        let if_then = format!(
          "(if {} then {}",
          cond.accept(self),
          then_branch.accept(self),
        );
        let else_ = if let Some(else_branch) = else_branch {
          format!(" else {})", else_branch.accept(self))
        } else {
          ")".to_string()
        };
        format!("{}{}", if_then, else_)
      }
      Stmt::Print { expr } => format!("(print {})", expr.accept(self)),
      Stmt::Return { keyword: _, value } => format!(
        "(return{})",
        if value.is_none() {
          "".to_string()
        } else {
          format!(" {}", value.as_ref().unwrap().accept(self))
        }
      ),
      Stmt::Variable { name, initializer } => stringify_variable(self, name, initializer),
      Stmt::While { cond, body } => format!("(while {} {})", cond.accept(self), body.accept(self)),
    }
  }

  fn visit_expr(&mut self, expr: &Expr) -> Self::Output {
    #[allow(unused_variables)]
    match expr {
      Expr::Assign { name, val } => format!("(assign {} {})", name.0, val.accept(self)),
      Expr::Binary { left, op, right } => {
        format!("({} {} {})", op.0, left.accept(self), right.accept(self))
      }
      Expr::Call { callee, args } => format!(
        "(call {} with [{}])",
        callee.accept(self),
        args
          .iter()
          .map(|a| a.accept(self))
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Expr::Get { obj, name } => format!("(get {}.{})", obj.accept(self), name.0),
      Expr::Grouping { expr } => format!("(group {})", expr.accept(self)),
      Expr::Literal { val } => format!("{:?}", val.0),
      Expr::Logical { left, op, right } => {
        format!("({} {} {})", op.0, left.accept(self), right.accept(self))
      }
      Expr::Set { obj, name, val } => {
        format!(
          "(set {}.{} <- {})",
          obj.accept(self),
          name.0,
          val.accept(self)
        )
      }
      Expr::Super { keyword: _, method } => format!("(super {})", method.0),
      Expr::This { keyword: _ } => format!("(this)"),
      Expr::Unary { op, right } => format!("({} {})", op.0, right.accept(self)),
      Expr::Var { name } => format!("(var {})", name.0),
    }
  }
}
