use super::expr::{LiteralValue, Expr};
use crate::{
  visit::{Visitor, VisitorAcceptor},
  STR,
};

pub type VariableField = (STR, Option<Expr>);
pub type FunctionFiled = (STR, Vec<STR>, Vec<Stmt>);

#[derive(Debug, Clone)]
pub enum Stmt {
  Block {
    stmts: Vec<Stmt>,
  },
  Class {
    name: STR,
    superclass: Option<VariableField>,
    methods: Vec<FunctionFiled>,
  },
  Expression {
    expr: Expr,
  },
  Function {
    name: STR,
    params: Vec<STR>,
    body: Vec<Stmt>,
  },
  If {
    cond: Expr,
    then_branch: Box<Stmt>,
    else_branch: Option<Box<Stmt>>,
  },
  Print {
    expr: Expr,
  },
  Return {
    keyword: LiteralValue,
    value: Option<Expr>,
  },
  Variable {
    name: STR,
    initializer: Option<Expr>,
  },
  While {
    cond: Expr,
    body: Box<Stmt>,
  },
}

impl VisitorAcceptor for Stmt {
  fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
    visitor.visit_stmt(self)
  }
}
