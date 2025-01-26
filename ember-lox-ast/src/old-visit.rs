use super::ast::prelude::*;

pub trait Visitor<T> {
  // 语句访问方法
  fn visit_expression_stmt(&mut self, expr: &Expr) -> T;
  fn visit_print_stmt(&mut self, expr: &Expr) -> T;
  fn visit_var_stmt(&mut self, name: &str, initializer: &Option<Expr>) -> T;
  // ...其他语句类型

  // 表达式访问方法
  fn visit_assign_expr(&mut self, name: &str, value: &Expr) -> T;
  fn visit_binary_expr(&mut self, left: &Expr, op: Operator, right: &Expr) -> T;
  fn visit_literal_expr(&mut self, value: &LiteralValue) -> T;
  // ...其他表达式类型
}

impl Stmt {
  pub fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
    match self {
      Stmt::Expression(expr) => visitor.visit_expression_stmt(expr),
      Stmt::Print(expr) => visitor.visit_print_stmt(expr),
      Stmt::Var(name, init) => visitor.visit_var_stmt(name, init),
      // ...其他分支处理
      _ => unimplemented!(),
    }
  }
}

impl Expr {
  pub fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
    match self {
      Expr::Assign(name, value) => visitor.visit_assign_expr(name, value),
      Expr::Binary(left, op, right) => visitor.visit_binary_expr(left, *op, right),
      Expr::Literal(value) => visitor.visit_literal_expr(value),
      // ...其他分支处理
      _ => unimplemented!(),
    }
  }
}

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
  fn visit_expression_stmt(&mut self, expr: &Expr) -> String {
    expr.accept(self)
  }

  fn visit_print_stmt(&mut self, expr: &Expr) -> String {
    format!("(print {})", expr.accept(self))
  }

  fn visit_var_stmt(&mut self, name: &str, initializer: &Option<Expr>) -> String {
    let init_str = initializer
      .as_ref()
      .map(|e| e.accept(self))
      .unwrap_or_default();
    format!("(var {} {})", name, init_str)
  }

  fn visit_assign_expr(&mut self, name: &str, value: &Expr) -> String {
    format!("(assign {} {})", name, value.accept(self))
  }

  fn visit_binary_expr(&mut self, left: &Expr, op: Operator, right: &Expr) -> String {
    format!("({} {} {})", op, left.accept(self), right.accept(self))
  }

  fn visit_literal_expr(&mut self, value: &LiteralValue) -> String {
    match value {
      LiteralValue::Number(n) => n.to_string(),
      LiteralValue::String(s) => s.to_string(),
      LiteralValue::Bool(b) => b.to_string(),
      LiteralValue::Nil => "nil".to_string(),
    }
  }
}

#[cfg(test)]
mod ast_printer_test {
  use super::*;

  #[test]
  fn case1() {
    // AST: 1 + 2 * 3
    let expr = Expr::Binary(
      Expr::Literal(LiteralValue::Number(1.0)).into(),
      Operator::Plus,
      Expr::Binary(
        Expr::Literal(LiteralValue::Number(2.0)).into(),
        Operator::Multiply,
        Expr::Literal(LiteralValue::Number(3.0)).into(),
      )
      .into(),
    );

    let mut printer = AstPrinter;
    let result = expr.accept(&mut printer);

    assert_eq!(result, "(+ 1 (* 2 3))");
  }
}
