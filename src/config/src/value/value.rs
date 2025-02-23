use std::fmt::Display;

use indexmap::IndexMap;

use super::index::Index;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  Void,

  IncludeCommand(String, String),

  String(String),

  Boolean(bool),

  FloatNumber(f32),
  UnsignedIntegerNumber(u32),
  SignedIntegerNumber(i32),

  Object(IndexMap<String, Value>),

  Array(Vec<Value>),
}

impl<T> std::ops::Index<T> for Value
where
  T: Index,
{
  type Output = Value;

  fn index(&self, index: T) -> &Self::Output {
    static VOID: Value = Value::Void;
    index.index_into(self).unwrap_or(&VOID)
  }
}

impl<T> std::ops::IndexMut<T> for Value
where
  T: Index + Display,
{
  fn index_mut(&mut self, index: T) -> &mut Value {
    let data = index.index_into_mut(self);
    match data {
      Some(c) => c,
      None => {
        panic!("No such element indexed by {}", index)
      },
    }
  }
}
