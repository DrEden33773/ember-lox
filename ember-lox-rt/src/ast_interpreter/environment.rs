use dashmap::{mapref::one::Ref, DashMap};
use ember_lox_ast::ast::prelude::*;
use std::sync::Arc;

pub type Value = LiteralValue;
type STR = Arc<str>;

#[derive(Debug, Default, Clone)]
pub struct Environment {
  values: DashMap<STR, Value>,
}

impl Environment {
  pub fn assign(&mut self, name: STR, value: Value) -> Option<Value> {
    if !self.values.contains_key(&name) {
      return None;
    }
    self.values.insert(name, value)
  }

  pub fn get(&self, name: &str) -> Option<Ref<STR, Value>> {
    self.values.get(name)
  }

  pub fn define(&mut self, name: STR, value: Value) {
    self.values.insert(name, value);
  }
}
