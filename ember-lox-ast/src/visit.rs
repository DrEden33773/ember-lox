pub mod prelude {
  pub use super::Visitor;
}

pub trait Visitor {
  type Output;
}
