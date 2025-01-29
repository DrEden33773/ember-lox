//! A `tree-walk` interpreter for the `Ember-lox` language.
//!
//! `Tree-walk` means that `NO BYTECODE` is generated, it will evaluate everything
//! recursively (from a valid entry point of `AST`) and return the result.  

use crate::error::report;
use ember_lox_ast::{
  ast::prelude::*,
  visit::{Visitor, VisitorAcceptor},
};
use environment::Env;
use std::cmp::{PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

pub mod environment;

#[derive(Default)]
pub struct Interpreter {
  env: Env,
  has_runtime_error: bool,
  is_in_repl: bool,
}

impl Interpreter {
  fn runtime_error<T>(&mut self) -> Option<T> {
    self.has_runtime_error = true;
    None
  }

  pub fn disable_repl_mode(&mut self) {
    self.is_in_repl = false;
  }

  pub fn enable_repl_mode(&mut self) {
    self.is_in_repl = true;
  }

  pub fn evaluate(&mut self, expr: &Expr) -> Option<LiteralValue> {
    expr.accept(self)
  }

  pub fn interpret(&mut self, roots: &[Stmt], is_in_repl: bool) -> Result<(), ()> {
    self.is_in_repl = is_in_repl;
    for root in roots {
      self.execute(root);
      if self.has_runtime_error {
        // Reset the flag for the next run.
        // (extremely useful in `REPL` mode)
        if is_in_repl {
          self.has_runtime_error = false;
        }
        self.disable_repl_mode();
        return Err(());
      }
    }
    self.disable_repl_mode();
    Ok(())
  }

  pub fn execute(&mut self, root: &Stmt) {
    root.accept(self);
  }

  pub fn execute_block(&mut self, stmts: &[Stmt]) {
    self.env.new_enclosed();
    for stmt in stmts {
      self.execute(stmt);
      if self.has_runtime_error {
        self.env.drop_innermost_scope();
        return;
      }
    }
    self.env.drop_innermost_scope();
  }
}

#[allow(unused_variables)]
impl Visitor for Interpreter {
  type Output = Option<LiteralValue>;

  fn visit_stmt(&mut self, stmt: &Stmt) -> Self::Output {
    use Stmt::*;

    match stmt {
      Block { stmts } => {
        self.execute_block(stmts);
        None // Blocks don't return a value.
      }
      Class {
        name,
        superclass,
        methods,
      } => todo!(),
      Expression { expr } => {
        let curr_val = expr.accept(self);
        if curr_val.is_none() {
          return self.runtime_error();
        } else if self.is_in_repl {
          println!("{}", curr_val.unwrap());
        }
        None // Don't return anything for script mode.
      }
      Function { name, params, body } => todo!(),
      If {
        cond,
        then_branch,
        else_branch,
      } => todo!(),
      Print { expr } => {
        let Some(val) = expr.accept(self) else {
          return self.runtime_error();
        };
        println!("{}", val);
        None // Print statements don't return a value.
      }
      Return { keyword: _, value } => todo!(),
      Variable { name, initializer } => {
        let mut val = LiteralValue::Nil;
        if let Some(expr) = initializer {
          match expr.accept(self) {
            Some(v) => val = v,
            None => return self.runtime_error(),
          }
        }
        // Define the var
        self.env.define(name.0.to_owned(), val);
        None // Variable declarations don't return a value.
      }
      While { cond, body } => todo!(),
    }
  }

  fn visit_expr(&mut self, expr: &Expr) -> Self::Output {
    use Expr::*;
    use Operator::*;

    match expr {
      Assign { name, val } => {
        let val = val.accept(self)?;
        let var_name = name.0.to_owned();
        let line = name.1;
        if self.env.assign(var_name.to_owned(), val.clone()).is_none() {
          return report(line, &format!("Undefined variable: '{}'.", var_name));
        }
        Some(val) // To enable something like `var a = 1; print a = 2;`
      }
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
      Var { name } => {
        let var_name = name.0.to_owned();
        let line = name.1;
        match self.env.get(&var_name) {
          Some(v) => Some(v.value().clone()),
          None => report(line, &format!("Undefined variable: '{}'.", var_name)),
        }
      }
    }
  }
}
