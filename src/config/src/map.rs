use std::{borrow::Borrow, collections::BTreeMap, hash::Hash};

use crate::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct IndexMap<K, V> {
  inner: BTreeMap<K, V>,
}

impl IndexMap<String, Value> {
  #[inline]
  pub fn new() -> Self {
    Self { inner: BTreeMap::<String, Value>::new() }
  }

  #[inline(always)]
  pub fn get<Q>(&self, key: &Q) -> Option<&Value>
  where
    String: Borrow<Q>,
    Q: Ord + ?Sized + Eq + Hash,
  {
    self.inner.get(key)
  }

  #[inline(always)]
  pub fn add(&mut self, key: String, content: Value) {
    self.inner.insert(key, content);
  }
}
