//! A `tree-walk` interpreter for the `Ember-lox` language.
//!
//! `Tree-walk` means that `NO BYTECODE` is generated, it will evaluate everything
//! recursively (from a valid entry point of `AST`) and return the result.  

use ember_lox_ast::{
  ast::prelude::*,
  visit::{Visitor, VisitorAcceptor},
};
use std::cmp::{PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

use crate::error::report;

#[derive(Default)]
pub struct Interpreter {
  has_runtime_error: bool,
}

impl Interpreter {
  pub fn interpret(&mut self, roots: &[Stmt]) -> Result<(), ()> {
    for root in roots {
      self.execute(root);
      if self.has_runtime_error {
        // Reset the flag for the next run.
        // (extremely useful in `REPL` mode)
        self.has_runtime_error = false;
        return Err(());
      }
    }
    Ok(())
  }

  pub fn execute(&mut self, root: &Stmt) {
    root.accept(self);
  }
}

#[allow(unused_variables)]
impl Visitor for Interpreter {
  type Output = Option<LiteralValue>;

  fn visit_stmt(&mut self, stmt: &Stmt) -> Self::Output {
    use Stmt::*;

    match stmt {
      Block { stmts } => todo!(),
      Class {
        name,
        superclass,
        methods,
      } => todo!(),
      Expression { expr } => {
        if expr.accept(self).is_none() {
          self.has_runtime_error = true;
        }
        None // Expressions don't return a value.
      }
      Function { name, params, body } => todo!(),
      If {
        cond,
        then_branch,
        else_branch,
      } => todo!(),
      Print { expr } => {
        let Some(val) = expr.accept(self) else {
          self.has_runtime_error = true;
          return None;
        };
        println!("{}", val);
        None // Print statements don't return a value.
      }
      Return { keyword: _, value } => todo!(),
      Variable { name, initializer } => todo!(),
      While { cond, body } => todo!(),
    }
  }

  fn visit_expr(&mut self, expr: &Expr) -> Self::Output {
    use Expr::*;
    use Operator::*;

    match expr {
      Assign { name, val } => todo!(),
      Binary { left, op, right } => {
        let left = left.accept(self)?;
        let right = right.accept(self)?;
        match op.0 {
          Plus => match left.add(&right) {
            Ok(r) => r.into(),
            Err(e) => report(op.1, &e),
          },
          Minus => match left.sub(&right) {
            Ok(r) => r.into(),
            Err(e) => report(op.1, &e),
          },
          Multiply => match left.mul(&right) {
            Ok(r) => r.into(),
            Err(e) => report(op.1, &e),
          },
          Divide => match left.div(&right) {
            Ok(r) => r.into(),
            Err(e) => report(op.1, &e),
          },
          Greater => match left.check_both_numeric(&right) {
            Ok((l, r)) => Some(l.gt(&r).into()),
            Err(e) => report(op.1, &e),
          },
          GreaterEqual => match left.check_both_numeric(&right) {
            Ok((l, r)) => Some(l.ge(&r).into()),
            Err(e) => report(op.1, &e),
          },
          Less => match left.check_both_numeric(&right) {
            Ok((l, r)) => Some(l.lt(&r).into()),
            Err(e) => report(op.1, &e),
          },
          LessEqual => match left.check_both_numeric(&right) {
            Ok((l, r)) => Some(l.le(&r).into()),
            Err(e) => report(op.1, &e),
          },
          Equal => Some(left.eq(&right).into()),
          NotEqual => Some(left.ne(&right).into()),
          _ => report(op.1, &format!("Invalid binary operator: {}", op.0)),
        }
      }
      Call { callee, args } => todo!(),
      Get { obj, name } => todo!(),
      Grouping { expr } => expr.accept(self),
      Literal { val } => Some(val.0.to_owned()),
      Logical { left, op, right } => todo!(),
      Set { obj, name, val } => todo!(),
      Super { keyword: _, method } => todo!(),
      This { keyword: _ } => todo!(),
      Unary { op, right } => {
        let right = right.accept(self)?;
        match op.0 {
          Minus => match right.neg() {
            Ok(r) => r.into(),
            Err(e) => report(op.1, &e),
          },
          Not => match right.not() {
            Ok(r) => r.into(),
            Err(e) => report(op.1, &e),
          },
          _ => report(op.1, &format!("Invalid unary operator: {}", op.0)),
        }
      }
      Var { name } => todo!(),
    }
  }
}
