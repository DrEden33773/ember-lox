use crate::{
  error::{report_detail, report_token},
  Token,
};
use ember_lox_ast::ast::prelude::*;
use ember_lox_tokenizer::prelude::*;

pub mod decl;
pub mod expr;
pub mod stmt;
pub mod util;

#[derive(Debug, Clone)]
pub struct Parser<'src> {
  /// `May` include [TokenizationError].
  ///
  /// `WON'T` include [TokenKind::Eof] at the tail.
  tokens: Vec<Token<'src>>,
  /// Current token index
  curr_token: usize,
  /// Current `line` number
  curr_line: usize,
  /// Whether the parser has encountered a parsing error.
  had_parsing_error: bool,
}

impl<'src> Parser<'src> {
  pub fn had_parsing_error(&self) -> bool {
    self.had_parsing_error
  }

  fn report_err_token(&self, err_msg: &str) {
    // We assume that `self.peek()`'s worst case is to get `None`.
    let mut err_token = self.peek();
    if let Some(e) = err_token {
      // However, in this case, we get the first next-line token, ignored the end of line
      if e.tag.line != self.curr_line {
        // Thus, we need to look back to the previous token.
        err_token = self.prev();
      }
    }
    report_detail(self.curr_line, err_token.map(|e| e.val), err_msg);
  }

  fn consume_by_kind(&mut self, kind: TokenKind, err_msg: &str) -> Option<&Token<'src>> {
    if self.check_kind(kind) {
      return self.advance();
    }
    self.had_parsing_error = true;
    self.report_err_token(err_msg);
    None
  }

  #[allow(dead_code)]
  fn consume_by_token(&mut self, token: Token, err_msg: &str) -> Option<&Token<'src>> {
    if self.check_token(token) {
      return self.advance();
    }
    self.had_parsing_error = true;
    self.report_err_token(err_msg);
    None
  }

  fn synchronize(&mut self) {
    self.advance();

    while !self.is_at_end() {
      if let Some(prev) = self.prev() {
        if prev.tag.kind == TokenKind::Semi {
          return;
        }
      }

      if self.check_token_in(&[
        Token::class_tok(),
        Token::fun_tok(),
        Token::var_tok(),
        Token::for_tok(),
        Token::if_tok(),
        Token::while_tok(),
        Token::print_tok(),
        Token::return_tok(),
      ]) {
        return;
      }

      self.advance();
    }
  }
}

#[allow(dead_code)]
impl<'src> Parser<'src> {
  /// Consumes the current token and returns it.
  fn advance(&mut self) -> Option<&Token<'src>> {
    let token = self.tokens.get(self.curr_token);
    if let Some(token) = token {
      self.curr_token += 1;
      // Remember to update the current line number.
      self.curr_line = token.tag.line;
    }
    token
  }
  /// Returns the current token we have yet to consume
  fn peek(&self) -> Option<&Token<'src>> {
    self.tokens.get(self.curr_token)
  }
  /// Returns the most recently consumed token.
  fn prev(&self) -> Option<&Token<'src>> {
    self.tokens.get(self.curr_token - 1)
  }
  /// Returns `true` if the parser has consumed all tokens.
  fn is_at_end(&self) -> bool {
    self.peek().is_none()
  }

  /// Returns `true` if the current token's kind is of the given one.
  ///
  /// Unlike [Parser::_match] this method does not consume the token.
  fn check_kind(&self, token_kind: TokenKind) -> bool {
    self.peek().map_or(false, |t| t.tag.kind == token_kind)
  }
  fn check_kind_in(&self, token_kinds: &[TokenKind]) -> bool {
    token_kinds.iter().any(|&k| self.check_kind(k))
  }
  /// Returns `true` if the current token is of the given one.
  ///
  /// Unlike [Parser::match_kind] this method does not consume the token.
  fn check_token(&self, token: Token) -> bool {
    self.peek().map_or(false, |&t| t == token)
  }
  fn check_token_in(&self, tokens: &[Token]) -> bool {
    tokens.iter().any(|&t| self.check_token(t))
  }

  /// Returns `true` if the current token's kind is of the given ones.
  ///
  /// Note that this method will consume the token if return `true`.
  fn match_kind_in(&mut self, token_kinds: &[TokenKind]) -> bool {
    token_kinds.iter().any(|&k| self.match_kind(k))
  }
  fn match_kind(&mut self, token_kind: TokenKind) -> bool {
    let is_matched = self.peek().map_or(false, |t| t.tag.kind == token_kind);
    if is_matched {
      self.advance();
    }
    is_matched
  }

  /// Returns `true` if the current token is of the given ones.
  ///
  /// Note that this method will consume the token if return `true`.
  fn match_token_in(&mut self, tokens: &[Token]) -> bool {
    tokens.iter().any(|&t| self.match_token(t))
  }
  fn match_token(&mut self, token: Token) -> bool {
    let is_matched = self.peek().map_or(false, |&t| t == token);
    if is_matched {
      self.advance();
    }
    is_matched
  }
}

impl<'src> Parser<'src> {
  pub fn new(token_stream: impl Iterator<Item = Token<'src>>) -> Self {
    Self {
      tokens: token_stream.collect(),
      curr_token: 0,
      had_parsing_error: false,
      curr_line: 1,
    }
  }

  /// Parsing entry.
  /// ```
  /// program → declaration* EOF ;
  /// ```
  pub fn parse(&mut self) -> Option<Vec<Stmt>> {
    let mut stmts = vec![];
    while !self.is_at_end() {
      stmts.push(self.declaration());
    }
    if self.had_parsing_error {
      None
    } else {
      stmts
        .into_iter()
        .filter_map(|stmt| stmt)
        .collect::<Vec<_>>()
        .into()
    }
  }
}
