//! The main parser interface.
//!
//! Acknowledgement:
//!
//! - [rustc_parse](https://github.com/rust-lang/rust/tree/master/compiler/rustc_parse)

use ember_lox_tokenizer::prelude::*;
use std::{collections::HashSet, fs, io, sync::LazyLock};

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

pub static RESERVED_WORDS: LazyLock<HashSet<&str>> = LazyLock::new(|| {
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
/// Unlike [TagToken], [Token] could hold the name (original value)
#[derive(Debug, Clone, Copy)]
pub struct Token<'src> {
  pub tag: TagToken,
  pub val: &'src str,
}

use LiteralKind::*;
use TokenKind::*;
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
        Some(_) => format!("{} {} null", val.to_lowercase(), val),
        None => format!("IDENTIFIER {} null", val),
      },
      Literal { kind } => match kind {
        Number {
          base: _,
          empty_frac,
        } => {
          let mut displayed =
            val.parse::<f64>().unwrap_or_default().to_string() + if empty_frac { ".0" } else { "" };
          if !empty_frac && !displayed.contains('.') {
            displayed += ".0";
          }
          format!("NUMBER {} {}", val, displayed)
        }
        Str { terminated: _ } => {
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
