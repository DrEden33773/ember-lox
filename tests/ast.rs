#[cfg(test)]
pub mod ast_printer_test {
  use ember_lox_ast::{ast::prelude::*, visit::VisitorAcceptor, AstPrinter};
  use ember_lox_rt::prelude::*;

  #[test]
  fn case1() {
    // var x = 1 + 2 * 3
    let expr = Stmt::Variable {
      name: (intern_string("x"), 1).into(),
      initializer: Expr::Binary {
        left: Expr::Literal {
          val: (1.0.into(), 1).into(),
        }
        .into(),
        op: (Operator::Plus, 1).into(),
        right: Expr::Binary {
          left: Expr::Literal {
            val: (2.0.into(), 1).into(),
          }
          .into(),
          op: (Operator::Multiply, 1).into(),
          right: Expr::Literal {
            val: (3.0.into(), 1).into(),
          }
          .into(),
        }
        .into(),
      }
      .into(),
    };

    let mut printer = AstPrinter;
    let res = expr.accept(&mut printer);

    assert_eq!(res, "(var x (+ 1.0 (* 2.0 3.0)))")
  }
}
