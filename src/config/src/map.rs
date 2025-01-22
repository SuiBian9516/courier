use std::collections::BTreeMap;

use crate::value::value::Value;

#[derive(Debug)]
pub struct Map<K, V> {
  inner: BTreeMap<K, V>,
}

impl Map<String, Value> {
  #[inline]
  pub fn new() -> Self {
    Self { inner: BTreeMap::<String, Value>::new() }
  }
}
