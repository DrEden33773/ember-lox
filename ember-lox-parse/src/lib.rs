//! The main parser interface.
//!
//! Acknowledgement:
//!
//! - [rustc_parse](https://github.com/rust-lang/rust/tree/master/compiler/rustc_parse)

use ember_lox_tokenizer::prelude::*;
use macros::gen_reserved_tok_methods;
use parser::Parser;
use std::{collections::HashSet, sync::LazyLock};

pub mod error;
pub mod parser;

pub mod prelude {
  pub use super::{new_parser_from_src_str, tag_to_named_tokens, Token, RESERVED_WORDS};
  pub use ember_lox_tokenizer::prelude::*;
}

use TokenKind::*;
/// `WON'T` include [Eof] at the tail.
///
/// Will filter out listed [TokenKind]:
///
/// [Eof], [Whitespace], [NewLine], [LineComment]
pub fn new_parser_from_src_str<'src>(str: &'src str) -> Parser<'src> {
  let tag_tokens = tokenize(str);
  let tokens = tag_to_named_tokens(str, tag_tokens);
  let meaningful_tokens =
    tokens.filter(|t| !matches!(t.tag.kind, Eof | Whitespace | NewLine | LineComment));
  Parser::new(meaningful_tokens)
}

/// [`Token`] = Named token (with `line` info)
///
/// Unlike [TagToken], [Token] could hold the name (original value)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'src> {
  pub tag: TagToken,
  pub val: &'src str,
}

impl<'src> Token<'src> {
  pub fn eof_tok(line: usize) -> Self {
    Token {
      tag: TagToken {
        kind: TokenKind::Eof,
        len: 0,
        line,
      },
      val: "",
    }
  }
}

/// Special identifies for the token.
pub static RESERVED_WORDS: LazyLock<HashSet<&str>> = LazyLock::new(|| {
  [
    "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print", "return", "super",
    "this", "true", "var", "while",
  ]
  .iter()
  .copied()
  .collect()
});

gen_reserved_tok_methods!([
  "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print", "return", "super",
  "this", "true", "var", "while"
]);

use LiteralKind::*;
use TokenizationError::*;

/// Transmute [TagToken] to [Token].
pub fn tag_to_named_tokens<'src>(
  src: &'src str,
  tag_tokens: impl Iterator<Item = TagToken>,
) -> impl Iterator<Item = Token<'src>> {
  let mut start = 0;
  tag_tokens.map(move |tag_token| {
    let end = start + tag_token.len;
    let slice = &src[start..end];
    let transmuted = Token {
      tag: tag_token,
      val: slice,
    };
    start = end;
    transmuted
  })
}

impl Token<'_> {
  pub fn dbg(&self) -> String {
    let val = self.val;
    match self.tag.kind {
      LineComment | NewLine | Whitespace => String::new(),
      Identifier => match RESERVED_WORDS.get(val) {
        Some(_) => format!("{} {} null", val.to_uppercase(), val),
        None => format!("IDENTIFIER {} null", val),
      },
      Literal { kind } => match kind {
        Number => {
          let mut displayed = val.parse::<f64>().unwrap_or_default().to_string();
          let empty_frac = displayed.len() == val.len();
          if empty_frac || !displayed.ends_with(".0") {
            displayed += ".0";
          }
          format!("NUMBER {} {}", val, displayed)
        }
        Str => {
          debug_assert!(val.starts_with('"') && val.ends_with('"'));
          let len = val.len();
          // According to the test case, lexeme should remove the quotes.
          format!("STRING {} {}", val, &val[1..len - 1])
        }
      },
      TokErr(tokenization_error) => match tokenization_error {
        InvalidIdent { line } => format!("[line {}] Error: Invalid identifier: {}", line, val),
        UnexpectedCharacter { ch, line } => {
          format!("[line {}] Error: Unexpected character: {}", line, ch)
        }
        UnterminatedString { line } => format!("[line {}] Error: Unterminated string.", line),
        UnknownPrefix { line } => format!("[line {}] Error: Unknown prefix: {}", line, val),
      },
      _ => self.tag.dbg_pure_tag(),
    }
  }
}
