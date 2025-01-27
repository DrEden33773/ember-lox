//! Low-lever `lox` tokenizer
//!
//! Acknowledgement:
//!
//! - [`rustc_lexer`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_lexer)
//! - [`Crafting Interpreters`](https://craftinginterpreters.com/)
//! - [`Build your own Interpreter`](https://app.codecrafters.io/courses/interpreter/overview)

pub mod cursor;

pub use cursor::Cursor;

pub mod prelude {
  pub use super::cursor::Cursor;
  pub use super::{tokenize, Base, LiteralKind, TagToken, TokenKind, TokenizationError};
}

/// [`TagToken`] = Tag-only Token
///
/// That means, no allocated string (gathered from source text) is held.
/// Instead, it records current tag-only token's `length`.
///
/// [`TokenKind::Literal`]'s actual name will be gathered in `parsing` stage.
#[derive(Debug, Clone, Copy)]
pub struct TagToken {
  pub kind: TokenKind,
  pub len: usize,
  pub line: usize,
}

impl core::cmp::Eq for TagToken {}

impl core::cmp::PartialEq for TagToken {
  fn eq(&self, other: &Self) -> bool {
    // DON'T compare line
    self.kind == other.kind && self.len == other.len
  }
}

impl TagToken {
  pub fn new(kind: TokenKind, len: usize, line: usize) -> Self {
    Self { kind, len, line }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
  /// A line comment, e.g. `// comment`.
  LineComment,

  /// Any whitespace character sequence
  Whitespace,

  /// An identifier or keyword (e.g. `if`, `else`)
  Identifier,

  /// NewLine sequence (Linux/MacOS=LF, Windows=CRLF, OSX=CR)
  NewLine,

  /// Literals, e,g, `123`, `123.45`, `"abc"`, `"a"`
  ///
  /// See [LiteralKind] for more details.
  Literal { kind: LiteralKind },

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

  /// `=`
  Eq,
  /// `==`
  EqEq,
  /// `!`
  Bang,
  /// `!=`
  BangEq,
  /// `<`
  Lt,
  /// `<=`
  LtEq,
  /// `>`
  Gt,
  /// `>=`
  GtEq,

  /// `-`
  Minus,
  /// `+`
  Plus,
  /// `*`
  Star,
  /// `/`
  Slash,

  /// Tokenization Error
  TokErr(TokenizationError),

  /// End of file
  Eof,
}

/// Tokenization Error (treated as a part of [TokenKind])
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenizationError {
  /// An identifier that is invalid because it contains emoji.
  InvalidIdent { line: usize },

  /// Unexpected Character
  ///
  /// It can't be expected by the tokenizer, e.g. "№"
  UnexpectedCharacter { ch: char, line: usize },

  /// Unterminated String (loss right `"`)
  ///
  /// e.g. `"Hello, World!`
  UnterminatedString { line: usize },

  /// An unknown literal prefix, like `foo#`, `foo'`, `foo"`. Excludes
  /// literal prefixes that contain emoji, which are considered "invalid".
  ///
  /// Note that only the
  /// prefix (`foo`) is included in the token, not the separator (which is
  /// tokenized as its own distinct token). In Rust 2021 and later, reserved
  /// prefixes are reported as errors; in earlier editions, they result in a
  /// (allowed by default) lint, and are treated as regular identifier
  /// tokens.
  UnknownPrefix { line: usize },
}

impl TokenizationError {
  pub fn line(&self) -> usize {
    match self {
      InvalidIdent { line } => *line,
      UnexpectedCharacter { line, .. } => *line,
      UnterminatedString { line } => *line,
      UnknownPrefix { line } => *line,
    }
  }
}

use unicode_properties::UnicodeEmoji;
use TokenKind::*;
use TokenizationError::*;

/// Enum representing the literal types supported by the tokenizer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
  /// `123`, `123.12` (all treated as `f64`).
  Number,
  /// `"abc"`, `"abc`
  Str,
}

use LiteralKind::*;

/// Base of numeric literal encoding according to its prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Base {
  /// Literal starts with "0b".
  Binary = 2,
  /// Literal starts with "0o".
  Octal = 8,
  /// Literal doesn't contain a prefix.
  Decimal = 10,
  /// Literal starts with "0x".
  Hexadecimal = 16,
}

/// Creates an iterator that produces tokens from the input string.
///
/// Note that `EOF` won't be produced by this iterator.
pub fn tokenize(input: &str) -> impl Iterator<Item = TagToken> + '_ {
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

/// True if `c` is valid as a first character of an identifier.
/// See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for
/// a formal definition of valid identifier name.
pub fn is_id_start(c: char) -> bool {
  // This is XID_Start OR '_' (which formally is not a XID_Start).
  c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}

/// True if `c` is valid as a non-first character of an identifier.
/// See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for
/// a formal definition of valid identifier name.
pub fn is_id_continue(c: char) -> bool {
  unicode_xid::UnicodeXID::is_xid_continue(c)
}

/// The passed string is lexically an identifier.
pub fn is_ident(string: &str) -> bool {
  let mut chars = string.chars();
  if let Some(start) = chars.next() {
    is_id_start(start) && chars.all(is_id_continue)
  } else {
    false
  }
}

/// Whether `c` is `LF (\n)`.
///
/// `CR (\r)` is not included, because even for windows,
/// with both `CR` and `LF` will a new line be recognized.
pub fn is_a_possible_new_line(c: char) -> bool {
  c == BACKSLASH_N
}

/// Whether `c` is a whitespace character (but not a new_line).
pub fn is_whitespace(c: char) -> bool {
  c.is_whitespace() && !is_a_possible_new_line(c)
}

/// `\n` (Unicode)
pub const BACKSLASH_N: char = '\u{000A}';
/// `\r` (Unicode)
pub const BACKSLASH_R: char = '\u{000D}';

impl Cursor<'_> {
  /// Parses a token from the input string.
  pub fn advance_token(&mut self) -> TagToken {
    let first_char = match self.bump() {
      Some(c) => c,
      None => return TagToken::new(TokenKind::Eof, 0, self.line()),
    };

    let token_kind = match first_char {
      BACKSLASH_N => {
        *self.line_mut() += 1;
        NewLine
      }

      '/' => match self.first() {
        '/' => self.line_comment(),
        _ => Slash,
      },
      '=' => match self.first() {
        '=' => self.equal_equal(),
        _ => Eq,
      },
      '!' => match self.first() {
        '=' => self.bang_equal(),
        _ => Bang,
      },
      '<' => match self.first() {
        '=' => self.less_equal(),
        _ => Lt,
      },
      '>' => match self.first() {
        '=' => self.greater_equal(),
        _ => Gt,
      },

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
      '-' => Minus,
      '+' => Plus,
      '*' => Star,

      // Whitespace (`new_line` is not included here)
      c if is_whitespace(c) => self.whitespace(),

      // Identifier (this should be checked after other variant that can
      // start as identifier).
      c if is_id_start(c) => self.ident_or_unknown_prefix(),

      // Numeric Literal
      c @ '0'..='9' => {
        let literal_kind = self.number(c);
        Literal { kind: literal_kind }
      }

      // String Literal, will take `"` into account of `str_len`
      '"' => {
        let terminated = self.double_quoted_string();
        let str_len = self.pos_within_token();
        if !terminated {
          return TagToken::new(
            TokErr(UnterminatedString { line: self.line() }),
            str_len,
            self.line(),
          );
        }
        let kind = Str;
        Literal { kind }
      }

      _ => TokErr(UnexpectedCharacter {
        ch: first_char,
        line: self.line(),
      }),
    };
    let res = TagToken::new(token_kind, self.pos_within_token(), self.line());
    // Remember to reset the consumed bytes length!
    self.reset_pos_within_token();
    res
  }
}

impl Cursor<'_> {
  fn whitespace(&mut self) -> TokenKind {
    debug_assert!(is_whitespace(self.prev()));
    self.eat_while(is_whitespace);
    Whitespace
  }

  fn line_comment(&mut self) -> TokenKind {
    debug_assert!(self.prev() == '/' && self.first() == '/');
    self.bump(); // Eat `/`.

    self.eat_while(|c| c != '\n');
    LineComment
  }

  /// Eats double-quoted string and returns true
  /// if string is terminated.
  fn double_quoted_string(&mut self) -> bool {
    debug_assert!(self.prev() == '"');
    while let Some(c) = self.bump() {
      match c {
        '"' => {
          return true;
        }
        '\\' if self.first() == '\\' || self.first() == '"' => {
          // Bump again to skip escaped character.
          self.bump();
        }
        _ => {}
      }
    }
    // End of file reached.
    false
  }

  fn ident_or_unknown_prefix(&mut self) -> TokenKind {
    debug_assert!(is_id_start(self.prev()));
    // Start is already eaten, eat the rest of identifier.
    self.eat_while(is_id_continue);
    // Known prefixes must have been handled earlier. So if
    // we see a prefix here, it is definitely an unknown prefix.
    match self.first() {
      '#' | '"' | '\'' => TokErr(UnknownPrefix { line: self.line() }),
      c if !c.is_ascii() && c.is_emoji_char() => self.invalid_ident(),
      _ => Identifier,
    }
  }

  fn invalid_ident(&mut self) -> TokenKind {
    // Start is already eaten, eat the rest of identifier.
    self.eat_while(|c| {
      const ZERO_WIDTH_JOINER: char = '\u{200d}';
      is_id_continue(c) || (!c.is_ascii() && c.is_emoji_char()) || c == ZERO_WIDTH_JOINER
    });
    // An invalid identifier followed by '#' or '"' or '\'' could be
    // interpreted as an invalid literal prefix. We don't bother doing that
    // because the treatment of invalid identifiers and invalid prefixes
    // would be the same.
    TokErr(InvalidIdent { line: self.line() })
  }

  fn number(&mut self, _first_digit: char) -> LiteralKind {
    debug_assert!('0' <= self.prev() && self.prev() <= '9');

    let base = Base::Decimal;

    // Eat integer part
    self.eat_decimal_digits();

    // Make sure the pattern is `<numeric>.<numeric>`
    if self.first() == '.' && self.second().is_digit(base as u32) {
      self.bump(); // Eat `.`
      self.eat_decimal_digits();
    }

    Number
  }

  /// `_` could appear in the sequence of numeric literals.
  ///
  /// E.g. `1_000_000`, `100_000.123_456`
  fn eat_decimal_digits(&mut self) -> bool {
    let mut has_digits = false;
    loop {
      match self.first() {
        '_' => {
          self.bump();
        }
        '0'..='9' => {
          has_digits = true;
          self.bump();
        }
        _ => break,
      }
    }
    has_digits
  }
}

impl Cursor<'_> {
  fn equal_equal(&mut self) -> TokenKind {
    debug_assert!(self.prev() == '=' && self.first() == '=');
    self.bump(); // Eat `=`.
    EqEq
  }

  fn bang_equal(&mut self) -> TokenKind {
    debug_assert!(self.prev() == '!' && self.first() == '=');
    self.bump(); // Eat `=`.
    BangEq
  }

  fn less_equal(&mut self) -> TokenKind {
    debug_assert!(self.prev() == '<' && self.first() == '=');
    self.bump(); // Eat `=`.
    LtEq
  }

  fn greater_equal(&mut self) -> TokenKind {
    debug_assert!(self.prev() == '>' && self.first() == '=');
    self.bump(); // Eat `=`.
    GtEq
  }
}

impl TokenKind {
  pub fn try_get_line(&self) -> Option<usize> {
    match self {
      TokErr(UnexpectedCharacter { line, .. }) => Some(*line),
      _ => None,
    }
  }
}

impl TagToken {
  pub fn is_err(&self) -> bool {
    matches!(self.kind, TokErr(_))
  }

  pub fn try_get_line(&self) -> Option<usize> {
    match self.kind {
      TokErr(e) => e.line().into(),
      _ => None,
    }
  }

  pub fn dbg_pure_tag(&self) -> String {
    let prefix = match self.kind {
      OpenParen => "LEFT_PAREN (",
      CloseParen => "RIGHT_PAREN )",
      OpenBrace => "LEFT_BRACE {",
      CloseBrace => "RIGHT_BRACE }",
      OpenBracket => "LEFT_BRACKET [",
      CloseBracket => "RIGHT_BRACKET ]",

      Semi => "SEMICOLON ;",
      Dot => "DOT .",
      Comma => "COMMA ,",

      Eq => "EQUAL =",
      EqEq => "EQUAL_EQUAL ==",
      Bang => "BANG !",
      BangEq => "BANG_EQUAL !=",
      Lt => "LESS <",
      LtEq => "LESS_EQUAL <=",
      Gt => "GREATER >",
      GtEq => "GREATER_EQUAL >=",

      Minus => "MINUS -",
      Plus => "PLUS +",
      Star => "STAR *",
      Slash => "SLASH /",

      Eof => "EOF ",
      _ => "",
    };
    prefix.to_string() + if prefix.is_empty() { "" } else { " null" }
  }
}
