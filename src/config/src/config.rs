use crate::{
  deserializer::{error::DeserializerError, lexer::Lexer, Deserializer},
  Value,
};

pub struct Config;

impl Config {
  pub fn deserialize(data: String) -> Result<Value, DeserializerError> {
    let lexer = Lexer::new(data);
    let deserializer = Deserializer::new(lexer);
    deserializer.parse()
  }
}
