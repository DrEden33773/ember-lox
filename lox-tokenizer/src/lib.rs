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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
  /// A line comment, e.g. `// comment`.
  LineComment,

  /// Any whitespace character sequence
  Whitespace,

  /// An identifier or keyword (e.g. `if`, `else`)
  Ident,

  /// NewLine sequence (Linux/MacOS=LF, Windows=CRLF, OSX=CR)
  NewLine,

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

  /// Unknown character (aka. unexpected character).
  ///
  /// It can't be expected by the tokenizer, e.g. "№"
  Unknown { ch: char, line: u32 },

  /// End of file
  Eof,
}

use TokenKind::*;

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

/// This is Pattern_White_Space.
///
/// Note that:
///
/// 1. This set is stable (ie, it doesn't change with different
/// Unicode versions), so it's ok to just hard-code the values.
///
/// 2. Detection for `new_line` has been moved to
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
          unreachable!()
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

      _ => Unknown {
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
      Unknown { line, .. } => Some(*line),
      _ => None,
    }
  }
}

impl Token {
  pub fn is_err(&self) -> bool {
    matches!(self.kind, Unknown { .. })
  }

  pub fn try_get_line(&self) -> Option<u32> {
    self.kind.try_get_line()
  }

  pub fn dbg(&self) -> String {
    if let Unknown { ch, line } = self.kind {
      return format!("[line {}] Error: Unexpected character: {}", line, ch);
    }

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

    if prefix.is_empty() {
      prefix.to_string()
    } else {
      prefix.to_string() + " null"
    }
  }
}
