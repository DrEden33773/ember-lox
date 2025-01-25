//! ### Acknowledgement
//!
//! - [`rustc_lexer/src/cursor.rs`](https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/cursor.rs)

use std::str::Chars;

/// Peekable iterator over a char sequence.
///
/// Next characters can be peeked via `first` method,
/// and position can be shifted forward via `bump` method.
pub struct Cursor<'src> {
  len_remaining: usize,
  /// Iterator over chars. Slightly faster than a &str.
  chars: Chars<'src>,
  prev_ch: char,
  line: u32,
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'src> Cursor<'src> {
  pub fn new(input: &'src str) -> Cursor<'src> {
    Cursor {
      len_remaining: input.len(),
      chars: input.chars(),
      prev_ch: EOF_CHAR,
      line: 1,
    }
  }

  pub fn as_str(&self) -> &str {
    self.chars.as_str()
  }

  /// Returns the last eaten symbol (or `'\0'` in release builds).
  /// (For debug assertions only.)
  pub fn prev(&self) -> char {
    if cfg!(debug_assertions) {
      self.prev_ch
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
  pub fn pos_within_token(&self) -> usize {
    self.len_remaining - self.chars.as_str().len()
  }

  /// Resets the number of bytes consumed to `0`.
  pub fn reset_pos_within_token(&mut self) {
    self.len_remaining = self.chars.as_str().len();
  }

  /// Moves to the next character.
  pub fn bump(&mut self) -> Option<char> {
    let c = self.chars.next()?;

    if cfg!(debug_assertions) {
      self.prev_ch = c;
    }

    Some(c)
  }

  /// Eats symbols while predicate returns true or until the end of file is reached.
  pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
    while predicate(self.first()) && !self.is_eof() {
      self.bump();
    }
  }

  pub fn line(&self) -> u32 {
    self.line
  }

  pub fn line_mut(&mut self) -> &mut u32 {
    &mut self.line
  }
}
