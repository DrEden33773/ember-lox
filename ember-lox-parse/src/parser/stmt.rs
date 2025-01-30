use super::*;

impl<'src> Parser<'src> {
  /// ```
  /// statement → exprStmt
  ///           |  forStmt
  ///           |  ifStmt
  ///           |  printStmt
  ///           |  whileStmt
  ///           |  block ;
  /// ```
  fn statement(&mut self) -> Option<Stmt> {
    if self.match_token(Token::if_tok()) {
      return self.if_stmt();
    }
    if self.match_token(Token::for_tok()) {
      return self.for_stmt();
    }
    if self.match_token(Token::print_tok()) {
      return self.print_stmt();
    }
    if self.match_token(Token::while_tok()) {
      return self.while_stmt();
    }
    if self.match_kind(TokenKind::OpenBrace) {
      return Stmt::Block {
        stmts: self.block()?,
      }
      .into();
    }

    self.expr_stmt()
  }

  /// ```
  /// forStmt → "for" "(" ( varDecl | exprStmt | ";" )
  ///            expression? ";"
  ///            expression? ")" statement ;
  /// ```
  fn for_stmt(&mut self) -> Option<Stmt> {
    self.consume_by_kind(TokenKind::OpenParen, "Expect '(' after 'for'.")?;

    let initializer = if self.match_token(Token::var_tok()) {
      self.var_decl() // will consume the trailing semicolon
    } else if self.match_kind(TokenKind::Semi) {
      None
    } else {
      Some(self.expr_stmt()?) // will consume the trailing semicolon
    };

    let cond = if !self.check_kind(TokenKind::Semi) {
      Some(self.expression()?)
    } else {
      None
    };
    self.consume_by_kind(TokenKind::Semi, "Expect ';' after loop condition.");

    let increment = if !self.check_kind(TokenKind::CloseParen) {
      Some(self.expression()?)
    } else {
      None
    };
    self.consume_by_kind(TokenKind::CloseParen, "Expect ')' after for clauses.");

    // manual de-sugaring of the for loop
    let mut body = self.statement()?;
    // increment the loop variable
    if let Some(increment) = increment {
      body = Stmt::Block {
        stmts: vec![body, Stmt::Expression { expr: increment }.into()],
      }
      .into();
    }
    // check the loop condition
    body = Stmt::While {
      cond: cond.map_or(
        // if no condition, then set it as true
        Expr::Literal {
          val: (true.into(), self.curr_line).into(),
        },
        |c| c,
      ),
      body: body.into(),
    }
    .into();
    // initialize the loop variable
    if let Some(initializer) = initializer {
      body = Stmt::Block {
        stmts: vec![initializer, body],
      }
      .into();
    }

    body.into()
  }

  /// ```
  /// ifStmt → "if" "(" expression ")" statement
  ///         ( "else" statement )? ;
  /// ```
  fn if_stmt(&mut self) -> Option<Stmt> {
    self.consume_by_kind(TokenKind::OpenParen, "Expect '(' after 'if'.")?;
    let cond = self.expression()?;
    self.consume_by_kind(TokenKind::CloseParen, "Expect ')' after if condition.");

    let then_branch = self.statement()?;
    let else_branch = if self.match_token(Token::else_tok()) {
      Some(self.statement()?)
    } else {
      None
    };

    Stmt::If {
      cond,
      then_branch: then_branch.into(),
      else_branch: else_branch.map(|stmt| stmt.into()),
    }
    .into()
  }

  /// ```
  /// whileStmt → "while" "(" expression ")" statement ;
  /// ```
  fn while_stmt(&mut self) -> Option<Stmt> {
    self.consume_by_kind(TokenKind::OpenParen, "Expect '(' after 'while'.")?;
    let cond = self.expression()?;
    self.consume_by_kind(TokenKind::CloseParen, "Expect ')' after while condition.");

    let body = self.statement()?;

    Stmt::While {
      cond,
      body: body.into(),
    }
    .into()
  }

  /// ```
  /// block → "{" declaration* "}" ;
  /// ```
  fn block(&mut self) -> Option<Vec<Stmt>> {
    let mut stmts = vec![];
    let mut failed_parsing_decl = false;
    while !self.check_kind(TokenKind::CloseBrace) && !self.is_at_end() {
      // To delay the raise of parsing error.
      if let Some(decl) = self.declaration() {
        stmts.push(decl);
      } else {
        failed_parsing_decl = true;
      }
    }
    self.consume_by_kind(TokenKind::CloseBrace, "Expect '}' after block.");
    if !failed_parsing_decl {
      stmts.into()
    } else {
      None
    }
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
  ///             |  statement ;
  /// ```
  pub(crate) fn declaration(&mut self) -> Option<Stmt> {
    if self.match_token(Token::var_tok()) {
      let var = self.var_decl();
      if var.is_none() {
        self.synchronize();
        return None;
      }
      return var; // Remember to return the `var_decl` statement.
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
  fn print_stmt(&mut self) -> Option<Stmt> {
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
