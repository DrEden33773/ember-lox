//! The Ember-lox Abstract Syntax Tree (AST) module.
//!
//! # Note
//!
//! This API is unstable.

pub mod ast;
pub mod visit;

use crate::visit::{Visitor, VisitorAcceptor};
use ast::expr::Expr;
use ast::stmt::Stmt;
use ember_lox_rt::prelude::*;

pub struct AstPrinter;

fn stringify_function(
  p: &mut AstPrinter,
  name: &STR,
  params: &Vec<STR>,
  body: &Vec<Stmt>,
) -> String {
  let params = params.join(", ");
  let body = body.iter().map(|s| s.accept(p)).collect::<Vec<_>>();
  let mut res = format!("(fun {}({}) ", name, params);
  let new_line = format!("\n{}", "".repeat(name.len()));
  for (i, stmt) in body.iter().enumerate() {
    match i {
      i if i == 0 => res += "::: ",
      i if i == body.len() - 1 => res += " └─ ",
      _ => res += " ├─ ",
    }
    res += stmt;
    res += if i == body.len() - 1 { ")" } else { &new_line };
  }
  res
}

fn stringify_variable(p: &mut AstPrinter, name: &STR, initializer: &Option<Expr>) -> String {
  let init_str = initializer
    .as_ref()
    .map(|e| e.accept(p))
    .unwrap_or_default();
  format!("(var {} {})", name, init_str)
}

impl Visitor for AstPrinter {
  type Output = String;

  fn visit_stmt(&mut self, stmt: &Stmt) -> Self::Output {
    match stmt {
      Stmt::Block { stmts } => {
        let stmts = stmts.iter().map(|s| s.accept(self)).collect::<Vec<_>>();
        let mut res = "(block ".to_string();
        let new_line = format!("\n{}", "".repeat(res.len()));
        for (i, stmt) in stmts.iter().enumerate() {
          match i {
            i if i == 0 => res += "::: ",
            i if i == stmts.len() - 1 => res += " └─ ",
            _ => res += " ├─ ",
          }
          res += stmt;
          res += if i == stmts.len() - 1 { ")" } else { &new_line };
        }
        res
      }
      Stmt::Class {
        name,
        superclass, // similar with Stmt::Variable
        methods,    // similar with Stmt::Function
      } => {
        let superclass = superclass
          .as_ref()
          .map(|(name, _)| name.to_string())
          .unwrap_or_default();
        let methods = methods
          .iter()
          .map(|(name, params, body)| stringify_function(self, name, params, body))
          .collect::<Vec<_>>();
        let mut res = if superclass.is_empty() {
          format!("(class {} ", name)
        } else {
          format!("(class {} extends {} ", name, superclass)
        };
        let new_line = format!("\n{}", "".repeat(res.len()));
        for (i, method) in methods.iter().enumerate() {
          match i {
            i if i == 0 => res += "::: ",
            i if i == methods.len() - 1 => res += " └─ ",
            _ => res += " ├─ ",
          }
          res += method;
          res += if i == methods.len() - 1 {
            ")"
          } else {
            &new_line
          };
        }
        res
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
      Expr::Assign { name, val } => format!("(assign {} {})", name, val.accept(self)),
      Expr::Binary { left, op, right } => {
        format!("({} {} {})", op, left.accept(self), right.accept(self))
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
      Expr::Get { obj, name } => format!("(get {}.{})", obj.accept(self), name),
      Expr::Grouping { expr } => format!("(group {})", expr.accept(self)),
      Expr::Literal { val } => val.to_string(),
      Expr::Logical { left, op, right } => {
        format!("({} {} {})", op, left.accept(self), right.accept(self))
      }
      Expr::Set { obj, name, val } => {
        format!(
          "(set {}.{} <- {})",
          obj.accept(self),
          name,
          val.accept(self)
        )
      }
      Expr::Super { keyword: _, method } => format!("(super {})", method),
      Expr::This { keyword: _ } => format!("(this)"),
      Expr::Unary { op, right } => format!("({} {})", op, right.accept(self)),
      Expr::Var { name } => format!("(var {})", name),
    }
  }
}
