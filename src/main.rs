use ember_lox_ast::visit::VisitorAcceptor;
use ember_lox_ast::AstPrinter;
use ember_lox_parse::prelude::*;
use ember_lox_rt::ast_interpreter::Interpreter;
use std::env;
use std::fs;

const TEST_MODE: bool = false;
const TEST_CMD: &str = "evaluate";
const TEST_FILENAME: &str = "test.lox";

fn main() {
  let args: Vec<String> = env::args().collect();
  if !TEST_MODE && args.len() < 3 {
    eprintln!("Usage: <loxc-path> <Commands> <filename>\n");
    eprintln!("Commands:");
    eprintln!("  tokenize   - Tokenize the source code");
    eprintln!("  parse      - Parse the source code");
    eprintln!("  run        - Run the source code");
    eprintln!("  evaluate   - Evaluate the source code");
    return;
  }

  let command = if TEST_MODE {
    TEST_CMD
  } else {
    args[1].as_str()
  };
  let filename = if TEST_MODE {
    TEST_FILENAME
  } else {
    args[2].as_str()
  };

  let mut src = fs::read_to_string(filename).unwrap_or_else(|_| {
    eprintln!("Failed to read file `{}`", filename);
    String::new()
  });
  // To make everything goes normal, trim the src first.
  src = src.trim().to_string();
  // `}` cannot followed by `;`
  if matches!(command, "parse" | "evaluate") && !src.ends_with(";") && !src.ends_with("}") {
    // Non-strict mode `REPL` allows the user to omit the semicolon.
    // But only `strict` strategy offered, so we add a trailing one.
    src.push(';');
  }

  match command {
    "tokenize" => {
      let mut pure_tokens = vec![];
      let mut tok_errors = vec![];
      let tagged_tokens = tokenize(&src);
      let tokens = tag_to_named_tokens(&src, tagged_tokens);

      for token in tokens {
        if token.tag.is_err() {
          tok_errors.push(token);
        } else {
          pure_tokens.push(token);
        }
      }
      let exit_code = if tok_errors.is_empty() { 0 } else { 65 };
      let mut last_line = if pure_tokens.is_empty() {
        0
      } else {
        pure_tokens.last().unwrap().tag.line
      };
      last_line = last_line.max(tok_errors.last().map(|e| e.tag.line).unwrap_or_default());
      pure_tokens.push(Token::eof_tok(last_line));

      tok_errors.iter().for_each(|e| eprintln!("{}", e.dbg()));
      pure_tokens
        .iter()
        .filter(|t| !t.dbg().is_empty())
        .for_each(|t| println!("{}", t.dbg()));

      std::process::exit(exit_code);
    }
    "parse" => {
      let mut parser = new_parser_from_src_str(&src);
      let Some(asts) = parser.parse() else {
        std::process::exit(65)
      };
      let mut printer = AstPrinter;
      asts
        .into_iter()
        .for_each(|stmt| println!("{}", stmt.accept(&mut printer)));
    }
    c if matches!(c, "run" | "evaluate") => {
      let mut parser = new_parser_from_src_str(&src);
      let Some(asts) = parser.parse() else {
        std::process::exit(65)
      };
      let mut interpreter = Interpreter::default();
      let repl_mode = c == "evaluate";
      if interpreter.interpret(&asts, repl_mode).is_err() {
        std::process::exit(70)
      }
    }
    _ => eprintln!("Unknown command: {}", command),
  }
}
