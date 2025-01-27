//! The Ember-lox Abstract Syntax Tree (AST) module.

pub mod expr;
pub mod stmt;

pub mod prelude {
  pub use super::{expr::*, stmt::*};
}
