#[cfg(test)]
pub mod ast_printer_test {
  use ember_lox_ast::{
    ast::{
      expr::{self, Expr},
      stmt::Stmt,
    },
    visit::VisitorAcceptor,
    AstPrinter,
  };
  use ember_lox_rt::prelude::*;

  #[test]
  fn case1() {
    // var x = 1 + 2 * 3
    let expr = Stmt::Variable {
      name: intern_string("x"),
      initializer: Expr::Binary {
        left: Expr::Literal {
          val: expr::LiteralValue::Number(1.0),
        }
        .into(),
        op: expr::Operator::Plus,
        right: Expr::Binary {
          left: Expr::Literal {
            val: expr::LiteralValue::Number(2.0),
          }
          .into(),
          op: expr::Operator::Multiply,
          right: Expr::Literal {
            val: expr::LiteralValue::Number(3.0),
          }
          .into(),
        }
        .into(),
      }
      .into(),
    };

    let mut printer = AstPrinter;
    let res = expr.accept(&mut printer);

    assert_eq!(res, "(var x (+ 1 (* 2 3)))")
  }
}
