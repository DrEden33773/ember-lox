use super::expr::{Expr, LiteralValue};
use crate::visit::{Visitor, VisitorAcceptor};
use ember_lox_rt::prelude::*;

/// Box<Stmt> => prevent recursive definition (infinite size)
#[derive(Debug, Clone)]
pub enum Stmt {
  Block(Block),
  Class(Class),
  Expression(Expression),
  Function(Function),
  If(If),
  Print(Print),
  Return(Return),
  Variable(Variable),
  While(While),
}

impl VisitorAcceptor for Stmt {
  fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
    visitor.visit_stmt(self)
  }
}

#[derive(Debug, Clone)]
pub struct Block {
  pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Class {
  pub name: STR,
  pub superclass: Option<Variable>,
  pub methods: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Expression {
  pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct Function {
  pub name: STR,
  pub params: Vec<STR>,
  pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct If {
  pub cond: Expr,
  pub then_branch: Box<Stmt>,
  pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, Clone)]
pub struct Print {
  pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct Return {
  pub keyword: LiteralValue,
  pub value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct Variable {
  pub name: STR,
  pub initializer: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct While {
  pub cond: Expr,
  pub body: Box<Stmt>,
}
