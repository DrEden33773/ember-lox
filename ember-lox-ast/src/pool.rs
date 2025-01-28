use dashmap::DashMap;
use std::sync::{Arc, LazyLock, Weak};

/// The maximum length of a string to be interned.
pub const MAX_INTERN_STR_LEN: usize = 128;
/// Immutable Str Reference
pub type STR = Arc<str>;

pub mod prelude {
  pub use super::{intern_string, MAX_INTERN_STR_LEN, STR};
}

/// `String pool` for interning strings.
///
/// Note that [`Weak<str>`] is used to avoid reference cycles.
///
/// While all the reference to an [`Arc`] disappears, the [`Weak`] reference of it will be dropped.
static STRING_POOL: LazyLock<DashMap<String, Weak<str>>> = LazyLock::new(DashMap::new);

/// Intern a string into the string pool.
pub fn intern_string(s: &str) -> Arc<str> {
  if s.len() > MAX_INTERN_STR_LEN {
    return Arc::from(s);
  }

  // Try to find the string in the pool first.
  if let Some(weak_ptr) = STRING_POOL.get(s) {
    // Found! Upgrade it into an `Arc` to actually hold the ownership for counting.
    if let Some(arc_ptr) = weak_ptr.upgrade() {
      return arc_ptr;
    }
  }

  // If the string is not in the pool, create a new `Arc<str>` and insert it.
  let new_arc = Arc::from(s);
  let new_weak = Arc::downgrade(&new_arc);
  STRING_POOL.insert(s.to_string(), new_weak);
  new_arc
}
