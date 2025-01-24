use lox_tokenizer::prelude::*;
use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
    println!();
    eprintln!("Usage: <loxc-path> tokenize <filename>\n");
    return;
  }

  let command = args[1].as_str();
  let filename = args[2].as_str();

  match command {
    "tokenize" => {
      let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file `{}`", filename);
        String::new()
      });

      let mut pure_tokens = vec![];
      let mut tokenizer_errors = vec![];
      for token in tokenize_with_eof(&file_contents) {
        if token.is_err() {
          tokenizer_errors.push(token);
        } else {
          pure_tokens.push(token);
        }
      }
      tokenizer_errors.sort_by_cached_key(|t| t.try_get_line().unwrap_or_default());

      for error in tokenizer_errors {
        eprintln!("{}", error.dbg());
      }
      for pure_token in pure_tokens {
        let info = pure_token.dbg();
        if !info.is_empty() {
          println!("{}", info);
        }
      }
    }
    _ => {
      eprintln!("Unknown command: {}", command);
    }
  }
}
