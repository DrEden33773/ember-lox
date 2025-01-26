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

pub struct AstPrinter;

impl Visitor for AstPrinter {
  type Output = String;

  fn visit_stmt(&mut self, stmt: &Stmt) -> Self::Output {
    #[allow(unused_variables)]
    match stmt {
      Stmt::Block { stmts } => todo!(),
      Stmt::Class {
        name,
        superclass,
        methods,
      } => todo!(),
      Stmt::Expression { expr } => expr.accept(self),
      Stmt::Function { name, params, body } => todo!(),
      Stmt::If {
        cond,
        then_branch,
        else_branch,
      } => todo!(),
      Stmt::Print { expr } => format!("(print {})", expr.accept(self)),
      Stmt::Return { name, value } => todo!(),
      Stmt::Variable { name, initializer } => {
        let init_str = initializer
          .as_ref()
          .map(|e| e.accept(self))
          .unwrap_or_default();
        format!("(var {} {})", name, init_str)
      }
      Stmt::While { cond, body } => todo!(),
    }
  }

  fn visit_expr(&mut self, expr: &Expr) -> Self::Output {
    #[allow(unused_variables)]
    match expr {
      Expr::Assign { name, val } => format!("(assign {} {})", name, val.accept(self)),
      Expr::Binary { left, op, right } => {
        format!("({} {} {})", op, left.accept(self), right.accept(self))
      }
      Expr::Call { callee, args } => todo!(),
      Expr::Get { obj, name } => todo!(),
      Expr::Grouping { expr } => format!("(group {})", expr.accept(self)),
      Expr::Literal { val } => val.to_string(),
      Expr::Logical { left, op, right } => {
        format!("({} {} {})", op, left.accept(self), right.accept(self))
      }
      Expr::Set { obj, name, val } => todo!(),
      Expr::Super { keyword, method } => todo!(),
      Expr::This { keyword } => todo!(),
      Expr::Unary { op, right } => format!("({} {})", op, right.accept(self)),
      Expr::Var { name } => todo!(),
    }
  }
}
