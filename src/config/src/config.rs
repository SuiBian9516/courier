use crate::{
  deserializer::{error::DeserializerError, lexer::Lexer, Deserializer},
  map::IndexMap,
  Value,
};

pub struct Config;

impl Config {
  pub fn deserialize(data: String) -> Result<IndexMap<String, Value>, DeserializerError> {
    let lexer = Lexer::new(data);
    let mut deserializer = Deserializer::new(lexer);
    deserializer.parse()
  }
}
