use crate::ast::{expr::Expr, stmt::Stmt};

pub trait Visitor {
  type Output;
  fn visit_stmt(&mut self, stmt: &Stmt) -> Self::Output;
  fn visit_expr(&mut self, expr: &Expr) -> Self::Output;
}

pub trait VisitorAcceptor {
  fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output;
  fn wrapped_accept<V: Visitor>(
    &self,
    visitor: &mut V,
    mut before: impl FnMut(),
    mut after: impl FnMut(),
  ) -> V::Output {
    before();
    let res = self.accept(visitor);
    after();
    res
  }
}
