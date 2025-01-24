use lox_tokenizer::prelude::*;
use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
    println!();
    eprintln!("Usage: <loxc-path> tokenize <filename>");
    return;
  }

  let command = &args[1];
  let filename = &args[2];

  match command.as_str() {
    "tokenize" => {
      let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
      });

      for token in tokenize_with_eof(&file_contents) {
        println!("{}", token.dbg());
      }
    }
    _ => {
      eprintln!("Unknown command: {}", command);
    }
  }
}
