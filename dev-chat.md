# Dev-chat (with r1)

## 1

我现在正在用 rust 开发一个编译器, 目前已有代码如下:

```rust
/// Special identifies for the token.
pub static RESERVED_WORDS: LazyLock<HashSet<&str>> = LazyLock::new(|| {
  [
    "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print", "return", "super",
    "this", "true", "var", "while",
  ]
  .iter()
  .copied()
  .collect()
});

/// [`Token`] = Named-Token
///
/// Unlike [TagToken], [Token] could hold the name (original value)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'src> {
  pub tag: TagToken,
  pub val: &'src str,
}
```

我目前已经实现了一些代码:

```rust
impl<'src> Token<'src> {
  pub fn and_tok() -> Self {
    Token {
      tag: TagToken {
        kind: Identifier,
        len: 3,
      },
      val: "and",
    }
  }
  pub fn class_tok() -> Self {
    Token {
      tag: TagToken {
        kind: Identifier,
        len: 5,
      },
      val: "class",
    }
  }
  pub fn else_tok() -> Self {
    Token {
      tag: TagToken {
        kind: Identifier,
        len: 4,
      },
      val: "else",
    }
  }
  pub fn false_tok() -> Self {
    Token {
      tag: TagToken {
        kind: Identifier,
        len: 5,
      },
      val: "false",
    }
  }
  pub fn for_tok() -> Self {
    Token {
      tag: TagToken {
        kind: Identifier,
        len: 3,
      },
      val: "for",
    }
  }
  pub fn fun_tok() -> Self {
    Token {
      tag: TagToken {
        kind: Identifier,
        len: 3,
      },
      val: "fun",
    }
  }
  pub fn if_tok() -> Self {
    Token {
      tag: TagToken {
        kind: Identifier,
        len: 2,
      },
      val: "if",
    }
  }
  pub fn nil_tok() -> Self {
    Token {
      tag: TagToken {
        kind: Identifier,
        len: 3,
      },
      val: "nil",
    }
  }
  /* ... */
}
```

但是这样手动实现实在有些过于折磨, 我想请你将这个实现过程, 包装成一个 `macro`.

这个 `macro` 应当接收一串 `&str`, 对每个 `&str` 都视作:

```rust
pub fn <str>_tok() -> Self {
  Token {
    tag: TagToken {
      kind: Identifier,
      len: <str>.len(),
    },
    val: <str>,
  }
}
```

从而简化 `impl` 块.

你既可以使用传统的 `声明宏`, 也可以引用 `quote` 从而构建 `过程宏`.
