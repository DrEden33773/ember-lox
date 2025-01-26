//! The main parser interface.
//!
//! Acknowledgement:
//!
//! - [rustc_parse](https://github.com/rust-lang/rust/tree/master/compiler/rustc_parse)

use ember_lox_tokenizer::prelude::*;
use std::{collections::HashSet, fs, io, ops::Deref, sync::LazyLock};

pub mod parser;

pub mod prelude {
  pub use super::{
    new_parser_from_file, new_parser_from_src_str, tag_to_named_tokens, Token, RESERVED_WORDS,
  };
  pub use ember_lox_tokenizer::prelude::*;
}

pub fn new_parser_from_file(path: &str) -> io::Result<()> {
  let src_str = fs::read_to_string(path)?;
  Ok(new_parser_from_src_str(&src_str))
}

pub fn new_parser_from_src_str(str: &str) {
  let tag_tokens = tokenize_with_eof(str);
  let _named_tokens = tag_to_named_tokens(str, tag_tokens);
}

pub static RESERVED_WORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
  [
    "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print", "return", "super",
    "this", "true", "var", "while",
  ]
  .iter()
  .copied()
  .collect()
});

/// [`Token`] = Named-Token
///
/// Unlike [TagToken], [Token] could hold the name for `identifier` or `literal`.
#[derive(Debug, Clone, Copy)]
pub enum Token<'src> {
  Tagged(TagToken),
  Valued(TagToken, &'src str),
}

impl<'src> Deref for Token<'src> {
  type Target = TagToken;

  /// A delegation from [Token] to [TagToken].
  fn deref(&self) -> &Self::Target {
    match self {
      Token::Tagged(tag_token) => tag_token,
      Token::Valued(tag_token, _) => tag_token,
    }
  }
}

use LiteralKind::*;
use TokenKind::*;
use TokenizationError::*;

pub fn tag_to_named_tokens<'src>(
  src: &'src str,
  tag_tokens: impl Iterator<Item = TagToken>,
) -> impl Iterator<Item = Token<'src>> {
  let mut start = 0;
  tag_tokens.map(move |tag_token| {
    let end = start + tag_token.len;
    let transmuted = match tag_token.kind {
      Identifier | TokErr(_) => {
        let slice = &src[start..end];
        Token::Valued(tag_token, slice)
      }
      _ => Token::Tagged(tag_token),
    };
    start = end;
    transmuted
  })
}

impl Token<'_> {
  pub fn dbg(&self) -> String {
    match self {
      Token::Valued(tag_token, value) => match tag_token.kind {
        Literal { kind } => match kind {
          Number {
            base: _,
            empty_frac,
          } => {
            let mut lexeme = value.parse::<f64>().unwrap_or_default().to_string()
              + if empty_frac { ".0" } else { "" };
            if !empty_frac && !lexeme.contains('.') {
              lexeme += ".0";
            }
            format!("NUMBER {} {}", value, lexeme)
          }
          Str { terminated: _ } => {
            debug_assert!(value.starts_with('"') && value.ends_with('"'));
            let len = value.len();
            // According to the test case, lexeme should remove the quotes.
            format!("STRING {} {}", value, &value[1..len - 1])
          }
        },
        TokErr(InvalidIdent { line }) => {
          format!("[line {}] Error: Invalid identifier: {}", line, value)
        }
        TokErr(UnknownPrefix { line }) => {
          format!("[line {}] Error: Unknown prefix: {}", line, value)
        }
        Identifier => match RESERVED_WORDS.get(value) {
          Some(_) => format!("{} {} null", value.to_uppercase(), value),
          None => format!("IDENTIFIER {} null", value),
        },
        _ => "".to_string(),
      },
      Token::Tagged(tag_token) => {
        match tag_token.kind {
          TokErr(UnexpectedCharacter { ch, line }) => {
            return format!("[line {}] Error: Unexpected character: {}", line, ch)
          }
          TokErr(UnterminatedString { line }) => {
            return format!("[line {}] Error: Unterminated string.", line)
          }
          _ => {}
        };

        let curr = match tag_token.kind {
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
        curr.to_string() + if curr.is_empty() { "" } else { " null" }
      }
    }
  }
}
