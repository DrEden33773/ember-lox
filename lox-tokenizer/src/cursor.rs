//! ### Acknowledgement
//!
//! - [`rustc_lexer/src/cursor.rs`](https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/cursor.rs)

use std::str::Chars;

/// Peekable iterator over a char sequence.
///
/// Next characters can be peeked via `first` method,
/// and position can be shifted forward via `bump` method.
pub struct Cursor<'a> {
  len_remaining: usize,
  /// Iterator over chars. Slightly faster than a &str.
  chars: Chars<'a>,
  #[cfg(feature = "debug_assertions")]
  prev: char,
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
  pub fn new(input: &'a str) -> Cursor<'a> {
    Cursor {
      len_remaining: input.len(),
      chars: input.chars(),
      #[cfg(feature = "debug_assertions")]
      prev: EOF_CHAR,
    }
  }

  pub fn as_str(&self) -> &str {
    self.chars.as_str()
  }

  /// Returns the last eaten symbol (or `'\0'` in release builds).
  /// (For debug assertions only.)
  pub fn prev(&self) -> char {
    if cfg!(feature = "debug_assertions") {
      self.prev
    } else {
      EOF_CHAR
    }
  }

  /// Peeks the next symbol from the input stream without consuming it.
  /// If requested position doesn't exist, `EOF_CHAR` is returned.
  /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
  /// it should be checked with `is_eof` method.
  pub fn first(&self) -> char {
    // `.next()` optimizes better than `.nth(0)`
    self.chars.clone().next().unwrap_or(EOF_CHAR)
  }

  /// Peeks the second symbol from the input stream without consuming it.
  pub fn second(&self) -> char {
    // `.next()` optimizes better than `.nth(1)`
    let mut iter = self.chars.clone();
    iter.next();
    iter.next().unwrap_or(EOF_CHAR)
  }

  /// Peeks the third symbol from the input stream without consuming it.
  pub fn third(&self) -> char {
    // `.next()` optimizes better than `.nth(1)`
    let mut iter = self.chars.clone();
    iter.next();
    iter.next();
    iter.next().unwrap_or(EOF_CHAR)
  }

  /// Checks if there is nothing more to consume.
  pub fn is_eof(&self) -> bool {
    self.chars.as_str().is_empty()
  }

  /// Returns amount of already consumed symbols.
  pub fn pos_within_token(&self) -> u32 {
    (self.len_remaining - self.chars.as_str().len()) as u32
  }

  /// Resets the number of bytes consumed to `0`.
  pub fn reset_pos_within_token(&mut self) {
    self.len_remaining = self.chars.as_str().len();
  }

  /// Moves to the next character.
  pub fn bump(&mut self) -> Option<char> {
    let c = self.chars.next()?;

    if cfg!(feature = "debug_assertions") {
      self.prev = c;
    }

    Some(c)
  }
}
