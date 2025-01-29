use dashmap::{mapref::one::Ref, DashMap};
use ember_lox_ast::ast::prelude::*;
use std::sync::Arc;

pub type Value = LiteralValue;
type STR = Arc<str>;

#[derive(Debug, Default, Clone)]
pub struct Environment {
  enclosing: Option<Box<Environment>>,
  values: DashMap<STR, Value>,
}

impl Environment {
  pub fn new_enclosed(enclosing: Environment) -> Self {
    Self {
      enclosing: Some(Box::new(enclosing)),
      values: DashMap::new(),
    }
  }

  pub fn assign(&mut self, name: STR, value: Value) -> Option<Value> {
    if !self.values.contains_key(&name) {
      // Recursively search for the variable in the enclosing environment.
      self.enclosing.as_mut()?.assign(name, value)
    } else {
      self.values.insert(name, value)
    }
  }

  pub fn get(&self, name: &str) -> Option<Ref<STR, Value>> {
    let curr_block = self.values.get(name);
    if curr_block.is_none() {
      // Recursively search for the variable in the enclosing environment.
      self.enclosing.as_ref()?.get(name)
    } else {
      curr_block
    }
  }

  pub fn define(&mut self, name: STR, value: Value) {
    self.values.insert(name, value);
  }
}
