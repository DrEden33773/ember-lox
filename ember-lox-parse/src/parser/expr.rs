use super::*;

const MAX_ARGS: usize = u8::MAX as usize;

impl<'src> Parser<'src> {
  /// ```
  /// expression → assignment ;
  /// ```
  pub(crate) fn expression(&mut self) -> Option<Expr> {
    self.assignment()
  }

  /// ```
  /// assignment → IDENTIFIER "=" assignment
  ///            |  logic_or ;
  /// ```
  fn assignment(&mut self) -> Option<Expr> {
    let expr = self.or()?;

    if self.match_kind(TokenKind::Eq) {
      let equal_token = self.prev().unwrap().to_owned();
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
  /// logic_or → logic_and ( "or" logic_and )* ;
  /// ```
  fn or(&mut self) -> Option<Expr> {
    let mut expr = self.and()?;

    while self.match_token(Token::or_tok()) {
      let or_op = self.prev().unwrap().to_owned();
      let right = self.and()?;
      expr = Expr::Logical {
        left: expr.into(),
        op: (Operator::Or, or_op.tag.line).into(),
        right: right.into(),
      }
    }

    Some(expr)
  }

  /// ```
  /// logic_and → equality ( "and" equality )* ;
  /// ```
  fn and(&mut self) -> Option<Expr> {
    let mut expr = self.equality()?;

    while self.match_token(Token::and_tok()) {
      let and_op = self.prev().unwrap().to_owned();
      let right = self.equality()?;
      expr = Expr::Logical {
        left: expr.into(),
        op: (Operator::And, and_op.tag.line).into(),
        right: right.into(),
      }
      .into();
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
  /// unary → ( "!" | "-" ) unary | call ;
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

    self.call()
  }

  /// ```
  /// call → primary ( "(" arguments? ")" )* ;
  /// ```
  fn call(&mut self) -> Option<Expr> {
    let mut expr = self.primary()?;

    loop {
      if self.match_kind(TokenKind::OpenParen) {
        expr = self.finish_call(expr.clone())?;
      } else {
        break;
      }
    }

    Some(expr)
  }

  /// ```
  /// arguments → expression ( "," expression )* ;
  /// ```
  fn finish_call(&mut self, callee: Expr) -> Option<Expr> {
    let mut args = vec![];

    if !self.check_kind(TokenKind::CloseParen) {
      loop {
        if args.len() >= MAX_ARGS {
          self.had_parsing_error = true;
          report_token(
            self.curr_line,
            self.peek(),
            &format!("Cannot have more than {} arguments.", MAX_ARGS),
          );
          // TODO: `return None` or `break`?
          return None;
        }
        args.push(self.expression()?);
        if !self.match_kind(TokenKind::Comma) {
          break;
        }
      }
    }

    self.consume_by_kind(TokenKind::CloseParen, "Expect ')' after arguments.")?;

    Expr::Call {
      callee: callee.into(),
      args,
    }
    .into()
  }

  /// ```
  /// primary → "true" | "false" | "nil"
  ///         |  NUMBER | STRING
  ///         |  "(" expression ")"
  ///         |  IDENTIFIER ;   
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

    if self.match_non_keyword_identifier() {
      let name = self.prev()?;
      return Expr::Var {
        name: (name.val, name.tag.line).into(),
      }
      .into();
    }
    if self.match_kind(TokenKind::OpenParen) {
      let expr = self.expression()?;
      self.consume_by_kind(TokenKind::CloseParen, "Expect ')' after expression.")?;
      return Expr::Grouping { expr: expr.into() }.into();
    }

    self.had_parsing_error = true;
    report_token(self.curr_line, self.peek(), "Expect expression.");
    None
  }
}
