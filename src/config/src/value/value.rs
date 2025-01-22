use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Value {
  Void,

  String(String),

  Boolean(bool),

  FloatNumber(f32),

  IntegerNumber(i32),

  Object(BTreeMap<String, Value>),

  Array(Vec<Value>),

  Pound,

  OpenBrace,
  CloseBrace,

  OpenParen,
  CloseParen,

  OpenBracket,
  CloseBracket,

  Semicolon,
  Comma,

  Reference,
  Dereference,

  Comment,

  End,
}
