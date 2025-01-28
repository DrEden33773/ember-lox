use super::*;

impl<'src> Parser<'src> {
  /// ```
  /// statement → exprStmt
  ///           |  printStmt ;
  /// ```
  pub(crate) fn statement(&mut self) -> Option<Stmt> {
    if self.match_token(Token::print_tok()) {
      return self.print();
    }
    self.expr_stmt()
  }

  /// ```
  /// exprStmt → expression ";" ;
  /// ```
  fn expr_stmt(&mut self) -> Option<Stmt> {
    let expr = self.expression()?;
    self.consume_by_kind(TokenKind::Semi, "Expect ';' after expression.");
    Stmt::Expression { expr }.into()
  }

  /// ```
  /// printStmt → "print" expression ";" ;
  /// ```
  fn print(&mut self) -> Option<Stmt> {
    let expr = self.expression()?;
    self.consume_by_kind(TokenKind::Semi, "Expect ';' after value.");
    Stmt::Print { expr }.into()
  }
}
