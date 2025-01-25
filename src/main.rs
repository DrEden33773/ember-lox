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
        if token.is_err() {
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
    _ => eprintln!("Unknown command: {}", command),
  }
}

#[cfg(test)]
mod test_enum_cast {
  use std::ops::{AddAssign, Deref, DerefMut};

  #[test]
  fn test_u8_repr_enum_to_u32() {
    enum Test {
      A = 1,
      B = 2,
      C = 3,
    }
    let a = Test::A as u32;
    let b = Test::B as u32;
    let c = Test::C as u32;
    assert_eq!(a, 1);
    assert_eq!(b, 2);
    assert_eq!(c, 3);
  }

  #[test]
  fn test_deref_delegation() {
    struct I32(i32);
    impl DerefMut for I32 {
      fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
      }
    }
    impl Deref for I32 {
      type Target = i32;
      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }
    impl From<i32> for I32 {
      fn from(value: i32) -> Self {
        Self(value)
      }
    }
    let mut integer: I32 = 42.into();
    integer.add_assign(32);
    assert_eq!(*integer, 74);
  }

  #[test]
  fn test_enum_debug_trait() {
    #[derive(Debug)]
    enum E {
      A,
      B,
    }
    assert_eq!(format!("{:?}", E::A), "A");
    assert_eq!(format!("{:?}", E::B), "B");
  }
}
