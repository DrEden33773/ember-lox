use dashmap::{mapref::one::Ref, DashMap};
use ember_lox_ast::ast::prelude::*;
use std::{collections::VecDeque, sync::Arc};

pub type Value = LiteralValue;
type STR = Arc<str>;

#[derive(Debug, Default, Clone)]
pub struct EnvFrame {
  values: DashMap<STR, Value>,
}

#[derive(Debug, Clone)]
pub struct Env {
  /// Direction: `Innermost -> ... -> Outermost`
  env_chain: VecDeque<EnvFrame>,
}

impl Default for Env {
  fn default() -> Self {
    Self::new()
  }
}

impl Env {
  pub fn new() -> Self {
    let mut env_chain = VecDeque::new();
    env_chain.push_front(EnvFrame::default());
    Self { env_chain }
  }

  pub fn new_enclosed(&mut self) {
    self.env_chain.push_front(EnvFrame::default());
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
