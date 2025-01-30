//! The Ember-lox Abstract Syntax Tree (AST) module.
//!
//! # Note
//!
//! This API is unstable.

pub mod ast;
pub mod pool;
pub mod visit;

use crate::visit::{Visitor, VisitorAcceptor};
use ast::expr::Expr;
use ast::stmt::Stmt;
use std::sync::Arc;

#[cfg(target_os = "windows")]
const NEWLINE_SEQ: &str = "\r\n";
#[cfg(not(target_os = "windows"))]
const NEWLINE_SEQ: &str = "\n";

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

#[derive(Default)]
pub struct AstPrinter;

impl AstPrinter {
  fn stringify_multi_lines(&mut self, starting: &str, contents: &[String], ending: &str) -> String {
    let len = contents.len();
    let contents = contents
      .iter()
      .enumerate()
      .map(|(i, s)| {
        // Note that `s` may contains `\n`, it should be splitted.
        // That's to update each lines indent, and concat it back to one.
        let indented = s
          .split(NEWLINE_SEQ)
          .map(|s| format!("    {}", s))
          .collect::<Vec<_>>()
          .join(NEWLINE_SEQ);
        format!("{}", indented) + if i != len - 1 { "" } else { ending }
      })
      .collect::<Vec<_>>();
    let mut vec = vec![starting.to_string() + "::"];
    vec.extend(contents);
    vec.join(NEWLINE_SEQ)
  }

  fn stringify_function(&mut self, name: &STR, params: &Vec<STR>, body: &Vec<Stmt>) -> String {
    let params = params
      .iter()
      .map(|s| s.0.as_ref())
      .collect::<Vec<_>>()
      .join(", ");
    let starting = if name.0.is_empty() {
      "(function ".to_string()
    } else {
      format!("(function {}({}) ", name.0, params)
    };
    let body = body
      .iter()
      .map(|s| {
        let res = s.accept(self);
        res
      })
      .collect::<Vec<_>>();
    self.stringify_multi_lines(&starting, &body, ")")
  }

  fn stringify_variable(&mut self, name: &STR, initializer: &Option<Expr>) -> String {
    let init_str = initializer
      .as_ref()
      .map(|e| e.accept(self))
      .unwrap_or("nil".to_string());
    format!("(var {} {})", name.0, init_str)
  }
}

impl Visitor for AstPrinter {
  type Output = String;

  fn visit_stmt(&mut self, stmt: &Stmt) -> Self::Output {
    use Stmt::*;

    match stmt {
      Block { stmts } => {
        let starting = "(block ";
        let stmts = stmts.iter().map(|s| s.accept(self)).collect::<Vec<_>>();
        self.stringify_multi_lines(starting, &stmts, ")")
      }
      Class {
        name,
        superclass, // similar with Stmt::Variable
        methods,    // similar with Stmt::Function
      } => {
        let superclass = superclass
          .as_ref()
          .map(|(name, _)| name.0.to_string())
          .unwrap_or_default();
        let starting = if superclass.is_empty() {
          format!("(class {} ", name.0)
        } else {
          format!("(class {} extends {} ", name.0, superclass)
        };

        let methods = methods
          .iter()
          .map(|(name, params, body)| self.stringify_function(name, params, body))
          .collect::<Vec<_>>();

        self.stringify_multi_lines(&starting, &methods, ")")
      }
      Expression { expr } => expr.accept(self),
      Function { name, params, body } => self.stringify_function(name, params, body),
      If {
        cond,
        then_branch,
        else_branch,
      } => {
        // let if_then = format!(
        //   "(if {} then {}",
        //   cond.accept(self),
        //   then_branch.accept(self),
        // );
        // let else_ = if let Some(else_branch) = else_branch {
        //   format!("\n else {})", else_branch.accept(self))
        // } else {
        //   ")".to_string()
        // };
        // format!("{}{}", if_then, else_)
        let if_then_starting = format!("(if {} then ", cond.accept(self));
        let then_content = then_branch.accept(self);
        if let Some(else_branch) = else_branch {
          let if_then = self.stringify_multi_lines(&if_then_starting, &[then_content], NEWLINE_SEQ);
          let else_content = else_branch.accept(self);
          let else_ = self.stringify_multi_lines(" else ", &[else_content], ")");
          format!("{}{}", if_then, else_)
        } else {
          let if_then = self.stringify_multi_lines(&if_then_starting, &[then_content], ")");
          format!("{}", if_then)
        }
      }
      Print { expr } => format!("(print {})", expr.accept(self)),
      Return { keyword: _, value } => format!(
        "(return{})",
        if value.is_none() {
          "".to_string()
        } else {
          format!(" {}", value.as_ref().unwrap().accept(self))
        }
      ),
      Variable { name, initializer } => {
        let str = self.stringify_variable(name, initializer);
        format!("{}", str)
      }
      While { cond, body } => {
        let starting = format!("(while {} ", cond.accept(self));
        let body = body.accept(self);
        self.stringify_multi_lines(&starting, &[body], ")")
      }
    }
  }

  fn visit_expr(&mut self, expr: &Expr) -> Self::Output {
    use Expr::*;

    match expr {
      Assign { name, val } => format!("(assign {} {})", name.0, val.accept(self)),
      Binary { left, op, right } => {
        format!("({} {} {})", op.0, left.accept(self), right.accept(self))
      }
      Call { callee, args } => format!(
        "(call {} with [{}])",
        callee.accept(self),
        args
          .iter()
          .map(|a| a.accept(self))
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Get { obj, name } => format!("(get {}.{})", obj.accept(self), name.0),
      Grouping { expr } => format!("(group {})", expr.accept(self)),
      Literal { val } => format!("{:?}", val.0),
      Logical { left, op, right } => {
        format!("({} {} {})", op.0, left.accept(self), right.accept(self))
      }
      Set { obj, name, val } => {
        format!(
          "(set {}.{} <- {})",
          obj.accept(self),
          name.0,
          val.accept(self)
        )
      }
      Super { keyword: _, method } => format!("(super {})", method.0),
      This { keyword: _ } => format!("(this)"),
      Unary { op, right } => format!("({} {})", op.0, right.accept(self)),
      Var { name } => format!("(var {})", name.0),
    }
  }
}
