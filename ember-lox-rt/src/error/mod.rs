//! Error reporters.

pub fn report<T>(line: usize, msg: &str) -> Option<T> {
  eprintln!("{msg}");
  eprintln!("[line {line}]");
  None
}
