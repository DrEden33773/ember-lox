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
    use Stmt::*;

    match stmt {
      Block(_block) => todo!(),
      Class(_class) => todo!(),
      Expression(expression) => expression.expr.accept(self),
      Function(_function) => todo!(),
      If(_if) => todo!(),
      Print(print) => format!("(print {})", print.expr.accept(self)),
      Return(_return) => todo!(),
      Variable(var) => {
        let init_str = var
          .initializer
          .as_ref()
          .map(|e| e.accept(self))
          .unwrap_or_default();
        format!("(var {} {})", var.name, init_str)
      }
      While(_while) => todo!(),
    }
  }

  fn visit_expr(&mut self, _expr: &Expr) -> Self::Output {
    match _expr {
      Expr::Assign(assign) => format!("(assign {} {})", assign.name, assign.val.accept(self)),
      Expr::Binary(binary) => format!(
        "({} {} {})",
        binary.op,
        binary.left.accept(self),
        binary.right.accept(self)
      ),
      Expr::Call(_call) => todo!(),
      Expr::Get(_get) => todo!(),
      Expr::Grouping(_grouping) => todo!(),
      Expr::Literal(literal) => literal.val.to_string(),
      Expr::Logical(_logical) => todo!(),
      Expr::Set(_set) => todo!(),
      Expr::Super(_super) => todo!(),
      Expr::This(_this) => todo!(),
      Expr::Unary(_unary) => todo!(),
      Expr::Var(_var) => todo!(),
    }
  }
}
