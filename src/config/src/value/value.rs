use crate::map::IndexMap;

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
