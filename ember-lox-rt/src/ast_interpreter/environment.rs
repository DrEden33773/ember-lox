use dashmap::{mapref::one::Ref, DashMap};
use ember_lox_ast::ast::prelude::*;
use std::{
  collections::LinkedList,
  sync::{Arc, Mutex},
};

pub type Value = LiteralValue;
type STR = Arc<str>;

#[derive(Debug, Default, Clone)]
pub struct EnvironmentFrame {
  pub(crate) values: DashMap<STR, Value>,
}

#[derive(Debug, Clone)]
pub struct Environment {
  /// Direction: Inner -> Outer
  env_chain: LinkedList<EnvironmentFrame>,
}

impl Default for Environment {
  fn default() -> Self {
    Self::new()
  }
}

impl Environment {
  pub fn new() -> Self {
    Self {
      env_chain: LinkedList::new(),
    }
  }

  pub fn new_enclosed(&mut self) {
    self.env_chain.push_front(EnvironmentFrame::default());
  }

  pub fn drop_innermost_scope(&mut self) {
    self.env_chain.pop_front();
  }

  pub fn assign(&mut self, name: STR, value: Value) -> Option<Value> {
    for env_node in &mut self.env_chain {
      if env_node.values.contains_key(&name) {
        return env_node.values.insert(name, value);
      }
    }
    None
  }

  pub fn get(&self, name: &str) -> Option<Ref<STR, Value>> {
    for env_node in &self.env_chain {
      if let Some(value) = env_node.values.get(name) {
        return Some(value);
      }
    }
    None
  }

  pub fn define(&mut self, name: STR, value: Value) {
    self.env_chain.front_mut().and_then(|env_node| {
      env_node.values.insert(name, value);
      Some(())
    });
  }
}
