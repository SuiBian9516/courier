use crate::{
  deserializer::{lexer::Lexer, Deserializer},
  Deserializable, DeserializableError,
};

/// Config operator
pub struct Config;

impl Config {
  /// Parse `Marquage` data from [`str`]
  /// 
  /// # Example
  /// ```rust
  /// # use config::Config;
  /// # use config::value::Value;
  /// 
  /// let instance: Value = Config::parse_from_string("name \"Jack\"; age 20;").unwrap();
  /// ```
  pub fn parse_from_string<'a,T>(data: &'a str) -> Result<T, DeserializableError> where T: Deserializable{
    let lexer = Lexer::new(data.to_string());
    let deserializer = Deserializer::new(lexer);
    match deserializer.parse(){
      Ok(val) => {
        T::deserialize(&val)
      },
      Err(e) =>{
        Err(DeserializableError::ParsingError(e))
      }
    }
  }
}
