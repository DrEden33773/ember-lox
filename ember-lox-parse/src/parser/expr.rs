use super::*;

impl<'src> Parser<'src> {
  /// ```
  /// expression → assignment ;
  /// ```
  pub(crate) fn expression(&mut self) -> Option<Expr> {
    self.assignment()
  }

  /// ```
  /// assignment → IDENTIFIER "=" assignment
  ///            | equality ;
  /// ```
  fn assignment(&mut self) -> Option<Expr> {
    let expr = self.equality()?;

    if self.match_kind(TokenKind::Eq) {
      let equal_token = self.prev()?.to_owned();
      let line = equal_token.tag.line;
      let val = self.assignment()?;

      if let Expr::Var { name } = expr {
        return Expr::Assign {
          name,
          val: val.into(),
        }
        .into();
      }

      self.had_parsing_error = true;
      report_token(line, Some(&equal_token), "Invalid assignment target.");
    }

    Some(expr)
  }

  /// ```
  /// equality → comparison ( ( "!=" | "==" ) comparison )* ;
  /// ```
  fn equality(&mut self) -> Option<Expr> {
    let mut expr = self.comparison()?;

    while self.match_kind_in(&[TokenKind::BangEq, TokenKind::EqEq]) {
      let tag = self.prev().unwrap().tag;
      let op: Operator = tag.kind.try_into().unwrap();
      let line = tag.line;
      let right = self.comparison()?;
      expr = Expr::Binary {
        left: expr.into(),
        op: (op, line).into(),
        right: right.into(),
      }
      .into();
    }

    Some(expr)
  }

  /// ```
  /// comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
  /// ```
  fn comparison(&mut self) -> Option<Expr> {
    let mut expr = self.term()?;

    while self.match_kind_in(&[
      TokenKind::Gt,
      TokenKind::GtEq,
      TokenKind::Lt,
      TokenKind::LtEq,
    ]) {
      let tag = self.prev().unwrap().tag;
      let op: Operator = tag.kind.try_into().unwrap();
      let line = tag.line;
      let right = self.term()?;
      expr = Expr::Binary {
        left: expr.into(),
        op: (op, line).into(),
        right: right.into(),
      }
      .into();
    }

    Some(expr)
  }

  /// ```
  /// term → factor ( ( "-" | "+" ) factor )* ;
  /// ```
  fn term(&mut self) -> Option<Expr> {
    let mut expr = self.factor()?;

    while self.match_kind_in(&[TokenKind::Minus, TokenKind::Plus]) {
      let tag = self.prev().unwrap().tag;
      let op: Operator = tag.kind.try_into().unwrap();
      let line = tag.line;
      let right = self.factor()?;
      expr = Expr::Binary {
        left: expr.into(),
        op: (op, line).into(),
        right: right.into(),
      }
      .into();
    }

    Some(expr)
  }

  /// ```
  /// factor → unary ( ( "/" | "*" ) unary )* ;
  /// ```
  fn factor(&mut self) -> Option<Expr> {
    let mut expr = self.unary()?;

    while self.match_kind_in(&[TokenKind::Slash, TokenKind::Star]) {
      let tag = self.prev().unwrap().tag;
      let op: Operator = tag.kind.try_into().unwrap();
      let line = tag.line;
      let right = self.unary()?;
      expr = Expr::Binary {
        left: expr.into(),
        op: (op, line).into(),
        right: right.into(),
      }
      .into();
    }

    Some(expr)
  }

  /// ```
  /// unary → ( "!" | "-" ) unary
  ///       | primary ;
  /// ```
  fn unary(&mut self) -> Option<Expr> {
    if self.match_kind_in(&[TokenKind::Bang, TokenKind::Minus]) {
      let tag = self.prev().unwrap().tag;
      let op: Operator = tag.kind.try_into().unwrap();
      let line = tag.line;
      let right = self.unary()?;
      return Expr::Unary {
        op: (op, line).into(),
        right: right.into(),
      }
      .into();
    }

    self.primary()
  }

  /// ```
  /// primary → "true" | "false" | "nil"
  ///         | NUMBER | STRING
  ///         | "(" expression ")"
  ///         | IDENTIFIER ;   
  /// ```
  fn primary(&mut self) -> Option<Expr> {
    use LiteralKind::*;
    use TokenKind::*;

    if self.match_token(Token::true_tok()) {
      return Expr::Literal {
        val: (true.into(), self.curr_line).into(),
      }
      .into();
    }
    if self.match_token(Token::false_tok()) {
      return Expr::Literal {
        val: (false.into(), self.curr_line).into(),
      }
      .into();
    }
    if self.match_token(Token::nil_tok()) {
      return Expr::Literal {
        val: (Option::<f64>::None.into(), self.curr_line).into(),
      }
      .into();
    }

    if self.match_kind(Literal { kind: Number }) {
      let num = self.prev().unwrap().val;
      return Expr::Literal {
        val: (
          num.parse::<f64>().unwrap_or_default().into(),
          self.curr_line,
        )
          .into(),
      }
      .into();
    }
    if self.match_kind(Literal { kind: Str }) {
      // This should contains `"` at the start and end.
      let string = self.prev().unwrap().val;
      debug_assert!(string.starts_with('"') && string.ends_with('"'));

      return Expr::Literal {
        val: (string[1..string.len() - 1].into(), self.curr_line).into(),
      }
      .into();
    }

    if self.match_kind(TokenKind::Identifier) {
      let name = self.prev()?;
      return Expr::Var {
        name: (name.val, name.tag.line).into(),
      }
      .into();
    }
    if self.match_kind(TokenKind::OpenParen) {
      let expr = self.expression()?;
      self.consume_by_kind(TokenKind::CloseParen, "Expect ')' after expression.");
      return Expr::Grouping { expr: expr.into() }.into();
    }

    self.had_parsing_error = true;
    report_token(self.curr_line, self.peek(), "Expect expression.");
    None
  }
}
