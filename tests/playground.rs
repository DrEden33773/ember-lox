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
