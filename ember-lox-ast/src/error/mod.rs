//! Error reporters.

pub fn report(line: usize, msg: &str) {
  eprintln!("{msg}");
  eprintln!("[line {line}]");
}
