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

pub struct Interpreter;

impl Interpreter {
  pub fn interpret<V: VisitorAcceptor>(root: &V) -> Option<LiteralValue> {
    let mut interpreter = Self;
    root.accept(&mut interpreter)
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
      Expression { expr } => todo!(),
      Function { name, params, body } => todo!(),
      If {
        cond,
        then_branch,
        else_branch,
      } => todo!(),
      Print { expr } => todo!(),
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
          Greater => Some(left.gt(&right).into()),
          GreaterEqual => Some(left.ge(&right).into()),
          Less => Some(left.lt(&right).into()),
          LessEqual => Some(left.le(&right).into()),
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
