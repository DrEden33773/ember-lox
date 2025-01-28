use ember_lox_parse::Token;
use ember_lox_tokenizer::prelude::*;

#[test]
fn test_gen_reserved_tok_methods() {
  let and_tok = Token::and_tok();
  assert_eq!(
    and_tok,
    Token {
      tag: TagToken {
        kind: TokenKind::Identifier,
        len: 3,
        line: 0,
      },
      val: "and",
    }
  );
}

#[test]
fn test_gen_reserved_tok_methods_2() {
  let or_tok = Token::or_tok();
  assert_eq!(
    or_tok,
    Token {
      tag: TagToken {
        kind: TokenKind::Identifier,
        len: 2,
        line: 0,
      },
      val: "or",
    }
  );
}
