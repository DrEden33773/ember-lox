//! ## Low-lever `lox` tokenizer
//!
//! ### Acknowledgement
//!
//! - [`rustc_lexer`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_lexer)
//! - [`Crafting Interpreters`](https://craftinginterpreters.com/)
//! - [`Build your own Interpreter`](https://app.codecrafters.io/courses/interpreter/overview)

pub mod cursor;
#[deprecated(note = "`Tokenization Error` has been treated as a special `TokenKind`")]
pub mod error;

pub use cursor::Cursor;

pub mod prelude {
  pub use super::{tokenize, tokenize_with_eof, Token, TokenKind};
}

#[derive(Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub len: u32,
}

impl Token {
  pub fn new(kind: TokenKind, len: u32) -> Self {
    Self { kind, len }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  /// A line comment, e.g. `// comment`.
  LineComment,

  /// Any whitespace character sequence
  Whitespace,

  /// An identifier or keyword (e.g. `if`, `else`)
  Ident,

  /// NewLine sequence (Linux/MacOS=LF, Windows=CRLF, OSX=CR)
  NewLine,

  /// Literals, e,g, `123`, `123.45`, `"abc"`, `"a"`
  ///
  /// See [LiteralKind] for more details.
  Literal {
    kind: LiteralKind,
    // suffix_start: u32,
  },

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

  /// Unexpected Character
  ///
  /// It can't be expected by the tokenizer, e.g. "№"
  UnexpectedCharacter { ch: char, line: u32 },

  /// Unterminated String (loss right `"`)
  ///
  /// e.g. `"Hello, World!`
  UnterminatedString { line: u32 },

  /// End of file
  Eof,
}

use TokenKind::*;

/// Enum representing the literal types supported by the tokenizer.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LiteralKind {
  /// `123`, `123.12` (all treated as `f64`).
  Number { base: Base, literal: String },
  /// `"abc"`, `"abc`
  Str { literal: String },
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
pub fn tokenize_with_eof(input: &str) -> impl Iterator<Item = Token> + '_ {
  // Note that EOF's length is always 0
  tokenize(input).chain(std::iter::once(Token::new(TokenKind::Eof, 0)))
}

/// True if `c` is considered a whitespace according to Rust language definition.
/// See [Rust language reference](https://doc.rust-lang.org/reference/whitespace.html)
/// for definitions of these classes.
///
/// Note that:
///
/// 1. This set is stable (ie, it doesn't change with different
/// Unicode versions), so it's ok to just hard-code the values.
///
/// 2. Detection for `new_line` has been moved out of this function.
pub fn is_whitespace(c: char) -> bool {
  matches!(
    c,
    // Usual ASCII suspects
    '\u{0009}'   // \t
        // | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        // | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        // | '\u{0085}'

        // Bidirectional markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
  )
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

/// `\n` (Unicode)
pub const BACKSLASH_N: char = '\u{000A}';
/// `\r` (Unicode)
pub const BACKSLASH_R: char = '\u{000D}';

impl Cursor<'_> {
  /// Parses a token from the input string.
  pub fn advance_token(&mut self) -> Token {
    let first_char = match self.bump() {
      Some(c) => c,
      None => return Token::new(TokenKind::Eof, 0),
    };

    let token_kind = match first_char {
      BACKSLASH_R => {
        if cfg!(target_os = "windows") {
          // On Windows, `\r\n` is a newline.
          if self.first() == BACKSLASH_N {
            self.bump();
            // update `line`
            *self.line_mut() += 1;
            NewLine
          } else {
            Whitespace
          }
        } else {
          // On Linux / MacOS (>10), `\r` is just a regular character.
          Whitespace
        }
      }

      BACKSLASH_N => {
        // On Windows, this case is `unreachable`
        #[cfg(all(debug_assertions, target_os = "windows"))]
        {
          unreachable!();
        }

        // On Linux / MacOS, `\n` is a newline.
        #[cfg(not(target_os = "windows"))]
        {
          *self.line_mut() += 1;
          NewLine
        }
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

      c if is_whitespace(c) => self.whitespace(),

      // Numeric Literal
      c @ '0'..='9' => {
        let literal_kind = self.number(c);
        Literal { kind: literal_kind }
      }

      // String Literal
      '"' => {
        let gathered = self.double_quoted_string();
        let str_len = self.pos_within_token();
        let Some(gathered) = gathered else {
          // Unterminated String (loss right `"`)
          return Token::new(UnterminatedString { line: self.line() }, str_len);
        };
        let kind = Str { literal: gathered };
        Literal { kind }
      }

      _ => UnexpectedCharacter {
        ch: first_char,
        line: self.line(),
      },
    };
    let res = Token::new(token_kind, self.pos_within_token());
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
  fn double_quoted_string(&mut self) -> Option<String> {
    debug_assert!(self.prev() == '"');
    let mut gathered = String::new();
    while let Some(c) = self.bump() {
      match c {
        '"' => {
          return Some(gathered);
        }
        '\\' if self.first() == '\\' || self.first() == '"' => {
          gathered.push('"');
          // Bump again to skip escaped character.
          self.bump();
        }
        _ => gathered.push(c),
      }
    }
    // End of file reached.
    None
  }

  fn number(&mut self, first_digit: char) -> LiteralKind {
    debug_assert!('0' <= self.prev() && self.prev() <= '9');
    let base = Base::Decimal;
    let mut literal = first_digit.to_string();

    // Eat integer part
    let int_part = self.eat_decimal_digits();
    literal.push_str(&int_part);

    // Make sure the pattern is `<numeric>.<numeric>`
    if self.first() == '.' && self.second().is_digit(base as u8 as u32) {
      self.bump(); // Eat `.`
      literal.push('.');

      let frac_part = self.eat_decimal_digits();
      literal.push_str(&frac_part);
    }

    Number { base, literal }
  }

  /// `_` could appear in the sequence of numeric literals.
  ///
  /// E.g. `1_000_000`, `100_000.123_456`
  fn eat_decimal_digits(&mut self) -> String {
    let mut gathered = String::new();
    loop {
      match self.first() {
        '_' => {
          self.bump();
        }
        c @ '0'..='9' => {
          self.bump();
          gathered.push(c);
        }
        _ => break,
      }
    }
    gathered
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
  pub fn try_get_line(&self) -> Option<u32> {
    match self {
      UnexpectedCharacter { line, .. } => Some(*line),
      _ => None,
    }
  }
}

pub fn number_literal_to_lexeme_str(literal: &str) -> String {
  if literal.split_once('.').is_none() {
    // pure i64, add `.0` at the tail
    return literal.to_string() + ".0";
  };

  // has `.`, treat as f64
  let mut lexeme = format!("{}", literal.parse::<f64>().unwrap_or_default());

  // if lose `.0`, add it back
  if !lexeme.ends_with(".0") {
    lexeme.push_str(".0");
  }

  lexeme
}

impl Token {
  pub fn is_err(&self) -> bool {
    matches!(
      self.kind,
      UnexpectedCharacter { .. } | UnterminatedString { .. }
    )
  }

  pub fn try_get_line(&self) -> Option<u32> {
    self.kind.try_get_line()
  }

  pub fn dbg(&self) -> String {
    if let UnexpectedCharacter { ch, line } = self.kind {
      return format!("[line {}] Error: Unexpected character: {}", line, ch);
    }

    match self.kind {
      UnexpectedCharacter { ch, line } => {
        return format!("[line {}] Error: Unexpected character: {}", line, ch)
      }
      UnterminatedString { line } => return format!("[line {}] Error: Unterminated string.", line),
      _ => {}
    }

    let curr = match self.kind {
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

    if !curr.is_empty() {
      return curr.to_string() + " null";
    }

    let curr = match &self.kind {
      Literal { kind } => match kind {
        Number { literal, .. } => format!(
          "NUMBER {} {}",
          literal,
          number_literal_to_lexeme_str(literal)
        ),
        Str { literal } => format!("STRING \"{}\" {}", literal, literal),
      },
      _ => "".to_string(),
    };

    curr
  }
}
