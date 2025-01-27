use ember_lox_parse::prelude::*;
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
  // let command = "tokenize";
  // let filename = "test.lox";

  match command {
    "tokenize" => {
      let src = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file `{}`", filename);
        String::new()
      });

      let mut pure_tokens = vec![];
      let mut tok_errors = vec![];
      let tagged_tokens = tokenize_with_eof(&src);
      let tokens = tag_to_named_tokens(&src, tagged_tokens);

      for token in tokens {
        if token.tag.is_err() {
          tok_errors.push(token);
        } else {
          pure_tokens.push(token);
        }
      }
      let exit_code = if tok_errors.is_empty() { 0 } else { 65 };

      tok_errors.iter().for_each(|e| eprintln!("{}", e.dbg()));
      pure_tokens
        .iter()
        .filter(|t| !t.dbg().is_empty())
        .for_each(|t| println!("{}", t.dbg()));

      std::process::exit(exit_code);
    }
    "parse" => unimplemented!(),
    "evaluate" => unimplemented!(),
    _ => eprintln!("Unknown command: {}", command),
  }
}
