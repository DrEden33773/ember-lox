use super::*;

impl<'src> Parser<'src> {
  /// ```
  /// expression → equality ;
  /// ```
  pub(crate) fn expression(&mut self) -> Option<Expr> {
    self.equality()
  }

  /// ```
  /// equality → comparison ( ( "!=" | "==" ) comparison )* ;
  /// ```
  fn equality(&mut self) -> Option<Expr> {
    let mut expr = self.comparison()?;

    while self.match_kind_in(&[TokenKind::BangEq, TokenKind::EqEq]) {
      let op: Operator = self.prev().unwrap().tag.kind.try_into().unwrap();
      let right = self.comparison()?;
      expr = Expr::Binary {
        left: expr.into(),
        op: op.into(),
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
      let op: Operator = self.prev().unwrap().tag.kind.try_into().unwrap();
      let right = self.term()?;
      expr = Expr::Binary {
        left: expr.into(),
        op: op.into(),
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
      let op: Operator = self.prev().unwrap().tag.kind.try_into().unwrap();
      let right = self.factor()?;
      expr = Expr::Binary {
        left: expr.into(),
        op: op.into(),
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
      let op: Operator = self.prev().unwrap().tag.kind.try_into().unwrap();
      let right = self.unary()?;
      expr = Expr::Binary {
        left: expr.into(),
        op: op.into(),
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
      let op: Operator = self.prev().unwrap().tag.kind.try_into().unwrap();
      let right = self.unary()?;
      return Expr::Unary {
        op: op.into(),
        right: right.into(),
      }
      .into();
    }

    self.primary()
  }

  /// ```
  /// primary → NUMBER | STRING | "true" | "false" | "nil"
  ///         | "(" expression ")" ;
  /// ```
  fn primary(&mut self) -> Option<Expr> {
    use LiteralKind::*;
    use TokenKind::*;

    if self.match_token(Token::true_tok()) {
      return Expr::Literal {
        val: LiteralValue::Bool(true),
      }
      .into();
    }
    if self.match_token(Token::false_tok()) {
      return Expr::Literal {
        val: LiteralValue::Bool(false),
      }
      .into();
    }
    if self.match_token(Token::nil_tok()) {
      return Expr::Literal {
        val: LiteralValue::Nil,
      }
      .into();
    }

    if self.match_kind(Literal { kind: Number }) {
      let num = self.prev().unwrap().val;
      return Expr::Literal {
        val: num.parse::<f64>().unwrap().into(),
      }
      .into();
    }
    if self.match_kind(Literal { kind: Str }) {
      let string = self.prev().unwrap().val;
      return Expr::Literal { val: string.into() }.into();
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
