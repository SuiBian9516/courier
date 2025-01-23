use crate::Value;

use super::{error::DeserializerError, position::Position, token::Token};

pub struct Lexer {
  pointer: usize,
  data: String,
  // (line, column)
  position: (usize, usize),
}

impl Lexer {
  pub fn new(data: String) -> Self {
    Self { pointer: 0, data, position: (1, 0) }
  }

  pub fn get(&mut self) -> Result<Token, DeserializerError> {
    if let Some(data) = self.advance(true) {
      match data {
        '#' => self.process_command(),
        '"' => self.process_string_with_double_quote(),
        '\'' => self.process_string_with_single_quote(),
        ';' => Ok(self.create_token(Value::Semicolon)),
        ',' => Ok(self.create_token(Value::Comma)),
        '[' => Ok(self.create_token(Value::OpenBracket)),
        ']' => Ok(self.create_token(Value::CloseBracket)),
        '{' => Ok(self.create_token(Value::OpenBrace)),
        '}' => Ok(self.create_token(Value::CloseBrace)),
        '(' => Ok(self.create_token(Value::OpenParen)),
        ')' => Ok(self.create_token(Value::CloseParen)),
        '&' => self.process_reference(),
        '*' => self.process_dereference(),
        '-' => self.process_signed_number(),
        't' => self.process_specific_sets(0),
        'f' => self.process_specific_sets(1),
        'v' => self.process_specific_sets(2),
        '/' => {
          if self.peek(1) == '/' {
            self.process_comment();
            Ok(self.create_token(Value::Comment))
          } else {
            self.process_raw_string()
          }
        },
        other => {
          if self.is_ascii_from_chars(other) {
            self.process_raw_string()
          } else if self.is_digit_from_chars(other) {
            self.process_number()
          } else {
            Err(DeserializerError::UnexpectedValue(other, self.position))
          }
        },
      }
    } else {
      Ok(self.create_token(Value::End))
    }
  }

  fn process_command(&mut self) -> Result<Token, DeserializerError> {
    let start_pos: usize = self.pointer;
    let mut state: bool = false;
    let mut command_name = String::from("");
    while let Some(data) = self.advance(false) {
      if self.is_ascii_from_chars(data) {
        if state {
          return Err(DeserializerError::UnexpectedValue(data, self.position));
        }
        continue;
      } else if data == ' ' || data == '\t' {
        if state {
          continue;
        }
        let command = &self.data[start_pos..self.pointer - 1];
        match command {
          "include" => {
            state = true;
            command_name = "include".to_string();
          },
          "define" => {
            todo!();
          },
          _ => {
            return Err(DeserializerError::InvalidCommand(command.to_string(), self.position));
          },
        }
      } else if data == '\r' || data == '\n' {
        return Err(DeserializerError::InvalidNewLine(self.position));
      } else if data == '"' {
        if !state {
          return Err(DeserializerError::UnexpectedValue(data, self.position));
        }
        match self.process_string_with_double_quote() {
          Ok(t) => {
            if let Value::String(content) = t.get_value() {
              return Ok(self.create_token(Value::Command(command_name, content.to_owned())));
            }
          },
          Err(e) => {
            return Err(e);
          },
        }
      }
    }
    Err(DeserializerError::UnexpectedTermination(self.position))
  }

  fn process_reference(&mut self) -> Result<Token, DeserializerError> {
    let start_pos: usize = self.pointer;
    while let Some(data) = self.advance(false) {
      if self.is_ascii_from_chars(data) {
        continue;
      } else if data == '_' || data == '-' {
        continue;
      } else if data == ' ' || data == '\t' || data == ',' || data == ';' || data == ']' {
        self.pointer -= 1;
        self.position.1 -= 1;
        return Ok(self.create_token(Value::Reference(self.data[start_pos..self.pointer].to_string())));
      } else {
        return Err(DeserializerError::UnexpectedValue(data, self.position));
      }
    }
    Err(DeserializerError::UnexpectedTermination(self.position))
  }

  fn process_dereference(&mut self) -> Result<Token, DeserializerError> {
    let start_pos: usize = self.pointer;
    while let Some(data) = self.advance(false) {
      if self.is_ascii_from_chars(data) {
        continue;
      } else if data == '_' || data == '-' {
        continue;
      } else if data == ' ' || data == '\t' || data == ',' || data == ';' || data == ']' {
        self.pointer -= 1;
        self.position.1 -= 1;
        return Ok(self.create_token(Value::Reference(self.data[start_pos..self.pointer].to_string())));
      } else {
        return Err(DeserializerError::UnexpectedValue(data, self.position));
      }
    }
    Err(DeserializerError::UnexpectedTermination(self.position))
  }

  fn process_string_with_double_quote(&mut self) -> Result<Token, DeserializerError> {
    //Previous check
    if let Some(data) = self.data.chars().nth(self.pointer - 2) {
      if data != ' ' && data != '\t' && data != '{' && data != '[' && data != ';' && data != ',' && data != '\r' && data != '\n' {
        return Err(DeserializerError::UnexpectedValue(data, self.position));
      }
    }
    let mut start_pos: usize = self.pointer;
    let mut buffer = Vec::<u8>::new();
    while let Some(data) = self.advance(false) {
      if data == '"' {
        buffer.extend_from_slice(self.data[start_pos..self.pointer - 1].as_bytes());
        let next_char = self.peek(1);
        if next_char != ',' && next_char != ';' && next_char != ' ' && next_char != '\t' && next_char != '\r' && next_char != '\n' && next_char != ']' {
          return Err(DeserializerError::UnexpectedValue(next_char, (self.position.0, self.position.1 + 1)));
        }
        return Ok(self.create_token(Value::String(String::from_utf8_lossy(&buffer).to_string())));
      } else if data == '\\' {
        match self.peek(1) {
          'n' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\n".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          'r' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\r".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          't' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\t".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '\\' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\\".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '"' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\"".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '\'' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("'".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '\0' => {
            return Err(DeserializerError::UnexpectedTermination(self.position));
          },
          other => {
            return Err(DeserializerError::UnexpectedValue(other, self.position));
          },
        }
      } else if data == '\n' || data == '\r' {
        return Err(DeserializerError::InvalidNewLine(self.position));
      } else {
        continue;
      }
    }
    Err(DeserializerError::UnexpectedTermination(self.position))
  }

  fn process_string_with_single_quote(&mut self) -> Result<Token, DeserializerError> {
    //Previous check
    if let Some(data) = self.data.chars().nth(self.pointer - 2) {
      if data != ' ' && data != '\t' && data != '{' && data != '[' && data != ';' && data != ',' && data != '\r' && data != '\n' {
        return Err(DeserializerError::UnexpectedValue(data, self.position));
      }
    }
    let mut start_pos: usize = self.pointer;
    let mut buffer = Vec::<u8>::new();
    while let Some(data) = self.advance(false) {
      if data == '\'' {
        buffer.extend_from_slice(self.data[start_pos..self.pointer - 1].as_bytes());
        let next_char = self.peek(1);
        if next_char != ',' && next_char != ';' && next_char != ' ' && next_char != '\t' && next_char != '\r' && next_char != '\n' && next_char != ']' {
          return Err(DeserializerError::UnexpectedValue(next_char, (self.position.0, self.position.1 + 1)));
        }
        return Ok(self.create_token(Value::String(String::from_utf8_lossy(&buffer).to_string())));
      } else if data == '\\' {
        match self.peek(1) {
          'n' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\n".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          'r' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\r".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          't' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\t".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '\\' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\\".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '"' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("\"".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '\'' => {
            buffer.extend_from_slice(&self.data[start_pos..self.pointer - 1].as_bytes());
            buffer.extend_from_slice("'".as_bytes());
            self.move_pointer_by(1);
            start_pos = self.pointer;
          },
          '\0' => {
            return Err(DeserializerError::UnexpectedTermination(self.position));
          },
          other => {
            return Err(DeserializerError::UnexpectedValue(other, self.position));
          },
        }
      } else if data == '\n' || data == '\r' {
        return Err(DeserializerError::InvalidNewLine(self.position));
      } else {
        continue;
      }
    }
    Err(DeserializerError::UnexpectedTermination(self.position))
  }

  fn process_raw_string(&mut self) -> Result<Token, DeserializerError> {
    let start_pos: usize = self.pointer - 1;
    while let Some(data) = self.advance(false) {
      match data {
        '/' => {
          if '/' == self.peek(1) {
            self.pointer -= 1;
            self.position.1 -= 1;
            return Ok(self.create_token(Value::String(self.data[start_pos..self.pointer].to_string())));
          } else {
            continue;
          }
        },
        ' ' => {
          self.pointer -= 1;
          self.position.1 -= 1;
          return Ok(self.create_token(Value::String(self.data[start_pos..self.pointer].to_string())));
        },
        '\t' => {
          self.pointer -= 1;
          self.position.1 -= 1;
          return Ok(self.create_token(Value::String(self.data[start_pos..self.pointer].to_string())));
        },
        '\n' => {
          self.pointer -= 1;
          self.position.1 -= 1;
          return Ok(self.create_token(Value::String(self.data[start_pos..self.pointer].to_string())));
        },
        '\r' => {
          self.pointer -= 1;
          self.position.1 -= 1;
          return Ok(self.create_token(Value::String(self.data[start_pos..self.pointer].to_string())));
        },
        ',' => {
          self.pointer -= 1;
          self.position.1 -= 1;
          return Ok(self.create_token(Value::String(self.data[start_pos..self.pointer].to_string())));
        },
        ';' => {
          self.pointer -= 1;
          self.position.1 -= 1;
          return Ok(self.create_token(Value::String(self.data[start_pos..self.pointer].to_string())));
        },
        ']' => {
          self.pointer -= 1;
          self.position.1 -= 1;
          return Ok(self.create_token(Value::String(self.data[start_pos..self.pointer].to_string())));
        },
        _ => {
          continue;
        },
      }
    }
    Ok(self.create_token(Value::String(self.data[start_pos..self.pointer].to_string())))
  }

  fn process_number(&mut self) -> Result<Token, DeserializerError> {
    let start_pos: usize = self.pointer - 1;
    while let Some(data) = self.advance(false) {
      if self.is_digit_from_chars(data) {
        continue;
      } else if data == '.' {
        return self.process_float_number(start_pos);
      } else if data == ' ' || data == '\t' || data == '\r' || data == '\n' || data == ']' || data == ',' || data == ';' {
        self.pointer -= 1;
        self.position.1 -= 1;
        return Ok(self.create_token(Value::UnsignedIntegerNumber((self.data[start_pos..self.pointer]).parse::<u32>().unwrap())));
      } else if data == '/' {
        if '/' == self.peek(1) {
          self.pointer -= 1;
          self.position.1 -= 1;
          return Ok(self.create_token(Value::UnsignedIntegerNumber(self.data[start_pos..self.pointer].parse::<u32>().unwrap())));
        } else {
          continue;
        }
      } else {
        self.move_pointer_to(start_pos + 1);
        return self.process_raw_string();
      }
    }
    Ok(self.create_token(Value::UnsignedIntegerNumber(self.data[start_pos..self.pointer].parse::<u32>().unwrap())))
  }

  fn process_signed_number(&mut self) -> Result<Token, DeserializerError> {
    let start_pos: usize = self.pointer - 1;
    while let Some(data) = self.advance(false) {
      if self.is_digit_from_chars(data) {
        continue;
      } else if data == '.' {
        return self.process_float_number(start_pos);
      } else if data == ' ' || data == '\t' || data == '\r' || data == '\n' || data == ']' || data == ',' || data == ';' {
        self.pointer -= 1;
        self.position.1 -= 1;
        return Ok(self.create_token(Value::SignedFloatNumber((self.data[start_pos..self.pointer]).parse::<i32>().unwrap())));
      } else if data == '/' {
        if '/' == self.peek(1) {
          self.pointer -= 1;
          self.position.1 -= 1;
          return Ok(self.create_token(Value::SignedFloatNumber(self.data[start_pos..self.pointer].parse::<i32>().unwrap())));
        } else {
          continue;
        }
      } else {
        self.move_pointer_to(start_pos + 1);
        return self.process_raw_string();
      }
    }
    Ok(self.create_token(Value::SignedFloatNumber(self.data[start_pos..self.pointer].parse::<i32>().unwrap())))
  }

  fn process_float_number(&mut self, start_pos: usize) -> Result<Token, DeserializerError> {
    while let Some(data) = self.advance(false) {
      if self.is_digit_from_chars(data) {
        continue;
      } else if data == ' ' || data == '\t' || data == '\r' || data == '\n' || data == ']' || data == ',' || data == ';' {
        self.pointer -= 1;
        self.position.1 -= 1;
        return Ok(self.create_token(Value::FloatNumber((self.data[start_pos..self.pointer]).parse::<f32>().unwrap())));
      } else if data == '/' {
        if '/' == self.peek(1) {
          self.pointer -= 1;
          self.position.1 -= 1;
          return Ok(self.create_token(Value::FloatNumber(self.data[start_pos..self.pointer].parse::<f32>().unwrap())));
        } else {
          continue;
        }
      } else {
        self.move_pointer_to(start_pos + 1);
        return self.process_raw_string();
      }
    }
    Ok(self.create_token(Value::FloatNumber(self.data[start_pos..self.pointer].parse::<f32>().unwrap())))
  }

  fn process_specific_sets(&mut self, sets: u8) -> Result<Token, DeserializerError> {
    /*
      Sets:
       - 0: true
       - 1: false
       - 2: void
       This function will match one by one.
       Only switch to string parsing function if it meets other characters.
    */
    let start_pos: usize = self.pointer;
    match sets {
      0 => {
        const SET: [char; 3] = ['r', 'u', 'e'];
        for c in SET {
          if let Some(data) = self.advance(false) {
            if data == c {
              continue;
            } else {
              self.move_pointer_to(start_pos);
              return self.process_raw_string();
            }
          } else {
            return Err(DeserializerError::UnexpectedTermination(self.position));
          }
        }
        let peeked = self.peek(1);
        if peeked == ',' || peeked == ';' || peeked == ' ' || peeked == '\t' || peeked == ']' || peeked == '\r' || peeked == '\n' {
          Ok(self.create_token(Value::Boolean(true)))
        } else if peeked == '/' && self.peek(2) == '/' {
          Ok(self.create_token(Value::Boolean(true)))
        } else {
          self.move_pointer_to(start_pos);
          self.process_raw_string()
        }
      },
      1 => {
        const SET: [char; 4] = ['a', 'l', 's', 'e'];
        for c in SET {
          if let Some(data) = self.advance(false) {
            if data == c {
              continue;
            } else {
              self.move_pointer_to(start_pos);
              return self.process_raw_string();
            }
          } else {
            return Err(DeserializerError::UnexpectedTermination(self.position));
          }
        }
        let peeked = self.peek(1);
        if peeked == ',' || peeked == ';' || peeked == ' ' || peeked == '\t' || peeked == ']' || peeked == '\r' || peeked == '\n' {
          Ok(self.create_token(Value::Boolean(false)))
        } else if peeked == '/' && self.peek(2) == '/' {
          Ok(self.create_token(Value::Boolean(false)))
        } else {
          self.move_pointer_to(start_pos);
          self.process_raw_string()
        }
      },
      2 => {
        const SET: [char; 3] = ['o', 'i', 'd'];
        for c in SET {
          if let Some(data) = self.advance(false) {
            if data == c {
              continue;
            } else {
              self.move_pointer_to(start_pos);
              return self.process_raw_string();
            }
          } else {
            return Err(DeserializerError::UnexpectedTermination(self.position));
          }
        }
        let peeked = self.peek(1);
        if peeked == ',' || peeked == ';' || peeked == ' ' || peeked == '\t' || peeked == ']' || peeked == '\r' || peeked == '\n' {
          Ok(self.create_token(Value::Void))
        } else if peeked == '/' && self.peek(2) == '/' {
          Ok(self.create_token(Value::Void))
        } else {
          self.move_pointer_to(start_pos);
          self.process_raw_string()
        }
      },
      _ => Err(DeserializerError::NoSetsFound(self.position)),
    }
  }

  fn process_comment(&mut self) {
    while let Some(data) = self.advance(false) {
      if data == '\n' || data == '\r' {
        self.pointer -= 1;
        self.position.1 -= 1;
        return;
      }
    }
  }

  #[inline]
  fn peek(&mut self, count: usize) -> char {
    /*
      It behaves like `advance` method, but do not move pointer
      and return exact data even if it meets end.
    */
    if let Some(data) = self.data.chars().nth(self.pointer + (count - 1)) {
      data
    } else {
      '\0'
    }
  }

  #[inline]
  fn advance(&mut self, skip_whitespace_and_newline: bool) -> Option<char> {
    /*
      It starts at 0.
      When called, it will read current data to which pointer points and move the pointer.

      It will add column by 1.
    */
    if !skip_whitespace_and_newline {
      let data = self.data.chars().nth(self.pointer);
      self.move_pointer_by(1);
      self.position.1 += 1;
      data
    } else {
      loop {
        let data = self.data.chars().nth(self.pointer);
        self.move_pointer_by(1);
        self.position.1 += 1;
        if let Some(cache) = data {
          if cache == ' ' || cache == '\t' {
            continue;
          } else if cache == '\n' {
            self.position.0 += 1;
            self.position.1 = 0;
          } else if cache == '\r' {
            if self.peek(1) == '\n' {
              self.move_pointer_by(1);
            }
            self.position.0 += 1;
            self.position.1 = 0;
          } else {
            return Some(cache);
          }
        } else {
          return None;
        }
      }
    }
  }

  #[inline(always)]
  fn create_token(&self, value: Value) -> Token {
    Token::new(value, Position::new(self.position.0, self.position.1))
  }

  #[inline(always)]
  fn move_pointer_by(&mut self, count: usize) {
    self.pointer += count;
  }

  #[inline(always)]
  fn move_pointer_to(&mut self, pos: usize) {
    self.pointer = pos;
  }

  #[inline(always)]
  fn is_digit_from_chars(&self, data: char) -> bool {
    data >= '0' && data <= '9'
  }

  #[inline(always)]
  fn is_ascii_from_chars(&self, data: char) -> bool {
    data >= 'a' && data <= 'z'
  }
}
