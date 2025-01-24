//! ## Low-lever `lox` tokenizer
//!
//! ### Acknowledgement
//!
//! - [`rustc_lexer`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_lexer)
//! - [`Crafting Interpreters`](https://craftinginterpreters.com/)
//! - [`Build your own Interpreter`](https://app.codecrafters.io/courses/interpreter/overview)

pub mod cursor;

pub use cursor::Cursor;

pub mod prelude {
  pub use super::{tokenize, tokenize_with_eof, Token, TokenKind};
}

#[derive(Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub line: u32,
}

impl Token {
  pub fn new(kind: TokenKind, line: u32) -> Self {
    Self { kind, line }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
  /// Any whitespace character sequence
  WhiteSpace,

  /// An identifier or keyword (e.g. `if`, `else`)
  Ident,

  /// `;`
  Semi,
  /// `,`
  Comma,
  /// `.`
  Dot,
  /// `(`
  OpenParen,
  /// `)`
  CloseParen,
  /// `{`
  OpenBrace,
  /// `}`
  CloseBrace,
  /// `[`
  OpenBracket,
  /// `]`
  CloseBracket,

  /// Unknown token, not expected by the lexer, e.g. "№"
  Unknown,

  /// End of file
  Eof,
}

use TokenKind::*;

impl Token {
  pub fn dbg(&self) -> String {
    let prefix = match self.kind {
      OpenParen => "LEFT_PAREN (",
      CloseParen => "RIGHT_PAREN )",
      Eof => "EOF ",
      _ => "UNKNOWN ",
    };
    prefix.to_string() + " null"
  }
}

/// Creates an iterator that produces tokens from the input string.
///
/// Note that `EOF` won't be produced by this iterator.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
  let mut cursor = Cursor::new(input);
  std::iter::from_fn(move || {
    let token = cursor.advance_token();
    if token.kind != TokenKind::Eof {
      Some(token)
    } else {
      None
    }
  })
}

/// Same with [tokenize], but produces an `EOF` token at the end.
#[cfg(feature = "debug_assertions")]
pub fn tokenize_with_eof(input: &str) -> impl Iterator<Item = Token> + '_ {
  // Note that EOF's line number is always 0
  // (since we could infer that from the last non-EOF token).
  tokenize(input).chain(std::iter::once(Token::new(TokenKind::Eof, 0)))
}

pub fn is_whitespace(c: char) -> bool {
  // This is Pattern_White_Space.
  //
  // Note that this set is stable (ie, it doesn't change with different
  // Unicode versions), so it's ok to just hard-code the values.

  matches!(
    c,
    // Usual ASCII suspects
    '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidirectional markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
  )
}

impl Cursor<'_> {
  /// Parses a token from the input string.
  pub fn advance_token(&mut self) -> Token {
    let first_char = match self.bump() {
      Some(c) => c,
      None => return Token::new(TokenKind::Eof, 0),
    };

    let token_kind = match first_char {
      // One-symbol tokens.
      ';' => Semi,
      ',' => Comma,
      '.' => Dot,
      '(' => OpenParen,
      ')' => CloseParen,
      '{' => OpenBrace,
      '}' => CloseBrace,
      '[' => OpenBracket,
      ']' => CloseBracket,

      _ => Unknown,
    };
    let res = Token::new(token_kind, self.pos_within_token());
    // Remember to reset the consumed bytes length!
    self.reset_pos_within_token();
    res
  }
}
