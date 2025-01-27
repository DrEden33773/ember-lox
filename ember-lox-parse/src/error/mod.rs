//! Reporting functions for errors.

use crate::Token;

pub fn report(line: usize, msg: &str) {
  eprintln!("[line {}] Error: {}", line, msg)
}

pub fn report_detail(line: usize, lexeme: Option<&str>, msg: &str) {
  match lexeme {
    Some(s) => eprintln!("[line {}] Error at '{}': {}", line, s, msg),
    None => eprintln!("[line {}] Error at end: {}", line, msg),
  }
}

pub fn report_token(line: usize, token: Option<&Token>, msg: &str) {
  match token {
    Some(t) => eprintln!("[line {}] Error at '{}': {}", line, t.val, msg),
    None => eprintln!("[line {}] Error at end: {}", line, msg),
  }
}
