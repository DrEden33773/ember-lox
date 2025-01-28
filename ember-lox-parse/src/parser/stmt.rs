use super::*;

impl<'src> Parser<'src> {
  /// ```
  /// statement → exprStmt
  ///           |  printStmt ;
  /// ```
  fn statement(&mut self) -> Option<Stmt> {
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
  /// declaration → varDecl
  ///             | statement ;
  /// ```
  pub(crate) fn declaration(&mut self) -> Option<Stmt> {
    if self.match_token(Token::var_tok()) {
      let var = self.var_decl();
      if var.is_none() {
        self.synchronize();
        return None;
      }
    }
    let stmt = self.statement();
    if stmt.is_none() {
      self.synchronize();
      return None;
    }
    stmt
  }

  /// ```
  /// printStmt → "print" expression ";" ;
  /// ```
  fn print(&mut self) -> Option<Stmt> {
    let expr = self.expression()?;
    self.consume_by_kind(TokenKind::Semi, "Expect ';' after value.");
    Stmt::Print { expr }.into()
  }

  /// ```
  /// varDecl → "var" IDENTIFIER ( "=" expression )? ";" ;
  /// ```
  fn var_decl(&mut self) -> Option<Stmt> {
    let name = self
      .consume_by_kind(TokenKind::Identifier, "Expect variable name")?
      .clone();

    let initializer = if self.match_kind(TokenKind::Eq) {
      Some(self.expression()?)
    } else {
      None
    };

    self.consume_by_kind(TokenKind::Semi, "Expect ';' after variable declaration.");
    Stmt::Variable {
      name: (name.val, name.tag.line).into(),
      initializer,
    }
    .into()
  }
}
