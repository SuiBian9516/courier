use crate::Value;

use super::position::Position;

#[derive(Debug)]
pub struct Token(Value, Position);

impl Token {
  pub fn new(value: Value, position: Position) -> Self {
    Self(value, position)
  }

  #[inline(always)]
  pub fn get_value(&self) -> &Value {
    &self.0
  }

  #[inline(always)]
  pub fn get_position(&self) -> &Position {
    &self.1
  }
}
