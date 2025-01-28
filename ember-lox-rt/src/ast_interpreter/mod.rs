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

pub struct Interpreter;

impl Interpreter {
  pub fn interpret<V: VisitorAcceptor>(root: &V) -> Result<LiteralValue, String> {
    let mut interpreter = Self;
    root.accept(&mut interpreter)
  }
}

#[allow(unused_variables)]
impl Visitor for Interpreter {
  type Output = Result<LiteralValue, String>;

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
        match op {
          Plus => left.add(&right),
          Minus => left.sub(&right),
          Multiply => left.mul(&right),
          Divide => left.div(&right),
          Greater => Ok(left.gt(&right).into()),
          GreaterEqual => Ok(left.ge(&right).into()),
          Less => Ok(left.lt(&right).into()),
          LessEqual => Ok(left.le(&right).into()),
          Equal => Ok(left.eq(&right).into()),
          NotEqual => Ok(left.ne(&right).into()),
          _ => Err(format!("Invalid binary operator: {}", op)),
        }
      }
      Call { callee, args } => todo!(),
      Get { obj, name } => todo!(),
      Grouping { expr } => expr.accept(self),
      Literal { val } => Ok(val.to_owned()),
      Logical { left, op, right } => todo!(),
      Set { obj, name, val } => todo!(),
      Super { keyword: _, method } => todo!(),
      This { keyword: _ } => todo!(),
      Unary { op, right } => {
        let right = right.accept(self)?;
        match op {
          Minus => right.neg(),
          Not => right.not(),
          _ => Err(format!("Invalid unary operator: {}", op)),
        }
      }
      Var { name } => todo!(),
    }
  }
}
