use super::{
  errors::scanner_error::ScannerError, literals::Literals, token::Token, token_type::TokenType,
};
use utils::tools::{is_digit_from_chars,is_ascii_from_chars};

pub struct Scanner {
  pointer: i32,
  line: i32,
  data: String,
  vec: Vec<Token>,
}

impl Scanner {
  pub fn new(data: String) -> Self {
    Self {
      pointer: 0,
      line: 1,
      data,
      vec: Vec::<Token>::new(),
    }
  }

  fn scan_token(&mut self) -> Result<&Vec<Token>, ScannerError> {
    while let Some(data) = self.advance() {
      match data {
        '#' => {
          self.add_token(TokenType::Pound, Literals::None, None, self.line);
        }
        '(' => {
          self.add_token(TokenType::OpenBrace, Literals::None, None, self.line);
        }
        ')' => {
          self.add_token(TokenType::CloseBrace, Literals::None, None, self.line);
        }
        '{' => {
          self.add_token(TokenType::OpenParen, Literals::None, None, self.line);
        }
        '}' => {
          self.add_token(TokenType::CloseParen, Literals::None, None, self.line);
        }
        '$' => {
          self.add_token(TokenType::VariableDefiner, Literals::None, None, self.line);
        }
        't'=>{
          if let Err(e) = self.check_by_characters('t') {
            return Err(e);
          }
        },
        'f'=>{
          if let Err(e) = self.check_by_characters('f') {
            return Err(e);
          }
        },
        'n'=>{
          if let Err(e) = self.check_by_characters('n') {
            return Err(e);
          }
        },
        '\u{0020}' => {}
        '\u{0009}' => {}
        '\u{000a}' => {
          if let Err(e) = self.process_new_line() {
            return Err(e);
          }
        }
        '\u{000d}' => {
          if let Err(e) = self.process_new_line() {
            return Err(e);
          }
        }
        '/' => {
          if '/' == self.peek() {
            self.parse_comment();
          } else {
            return Err(ScannerError::UnexpectedValue("/".to_string()));
          }
        }
        other => {
          if is_digit_from_chars(other){
            if let Err(e) = self.parse_number(){
              return Err(e);
            }
          }else if other == '"'{
            if let Err(e) = self.parse_string(){
              return Err(e);
            }
          }else if is_ascii_from_chars(other){
            if let Err(e) = self.parse_ascii_string(){
              return Err(e);
            }
          }else{
            return Err(ScannerError::UnexpectedValue(other.to_string()));
          }
        }
      };
    }

    self.add_token(TokenType::EOF, Literals::None, None, self.line);

    Ok(&self.vec)
  }

  fn parse_ascii_string(&mut self)->Result<(),ScannerError>{
    let start = (self.get_pointer_position() - 1) as usize;
    while let Some(data) = self.advance(){
      if is_ascii_from_chars(data){
        continue;
      }else{
        if data == '\n' || data == '\r'{
          if let Err(e) = self.process_new_line(){
            return Err(e);
          }
        }
        let value = &self.data[start.. (self.get_pointer_position() - 1) as usize];
        self.add_token(TokenType::AsciiString, Literals::AsciiString(value.to_string()), Some(value.to_string()), self.line);
        return Ok(());
      }
    }
    let value = &self.data[start.. self.get_pointer_position() as usize];
    self.add_token(TokenType::AsciiString, Literals::AsciiString(value.to_string()), Some(value.to_string()), self.line);
    Ok(())
  }

  fn parse_number(&mut self)->Result<(),ScannerError>{
    let start = (self.get_pointer_position() - 1) as usize;
    while let Some(data) = self.advance(){
      if data == '.'{
        if !is_digit_from_chars(self.peek()){
          return Err(ScannerError::IncompleteNumber);
        }else{
          self.jump(1);
        }
      }else if data == '\u{0020}' || data == '\u{0009}'{
        let value = &self.data[start.. (self.get_pointer_position() - 1) as usize];
        self.add_token(TokenType::Number, Literals::Number(value.parse::<f64>().unwrap()), Some(value.to_string()), self.line);
        return Ok(());
      }else if data == '\u{000a}' || data == '\u{000d}'{
        if let Err(e) = self.process_new_line() {
          return Err(e);
        }else {
          let value = &self.data[start.. (self.get_pointer_position() - 1) as usize];
          self.add_token(TokenType::Number, Literals::Number(value.parse::<f64>().unwrap()), Some(value.to_string()), self.line - 1);
          return Ok(());
        }
      }else if is_digit_from_chars(data){
        continue;
      }else {
        return Err(ScannerError::UnexpectedValue(data.to_string()));
      }
    }
    let value = &self.data[start.. (self.get_pointer_position() - 1) as usize];
    self.add_token(TokenType::Number, Literals::Number(value.parse::<f64>().unwrap()), Some(value.to_string()), self.line);
    Ok(())
  }

  fn parse_string(&mut self)->Result<(),ScannerError>{
    let mut cache = String::new();
    while let Some(data) = self.advance() {
      if data == '"'{
        self.add_token(TokenType::String, Literals::String(cache.clone()), Some(cache.clone()), self.line);
        return Ok(());
      }else if data == '\\'{
        if let Some(following) = self.advance() {
          match following{
            'n'=>cache.push_str("\n"),
            'r'=>cache.push_str("\r"),
            't'=>cache.push_str("\t"),
            '"'=>cache.push_str("\""),
            other=>{
              return Err(ScannerError::UnexpectedValue(other.to_string()));
            }
          }
        }else{
          return Err(ScannerError::UnexpectedTermination);
        }
      }else if data == '\n' || data == '\r'{
        return Err(ScannerError::UnexpectedStringTermination);
      }else{
        cache.push(data);
      }
    }
    Err(ScannerError::UnexpectedStringTermination)
  }

  fn parse_comment(&mut self)-> (){
    while let Some(data) = self.advance(){
      if data == '\u{000a}' || data == '\u{000d}'{
        let _ = self.process_new_line();
        break;
      }
    }
  }

  /* ==========Tool method========== */
  fn jump_to(&mut self,location:i32)->(){
    self.pointer = location;
  }

  fn check_by_characters(&mut self,first_character:char)->Result<(),ScannerError>{
    let start = self.get_pointer_position();
    if first_character == 'n'{
      let data:[char;3] = ['u','l','l'];
      for d in data.into_iter(){
        if let Some(cache) = self.advance() {
          if cache != d{
            if is_ascii_from_chars(cache){
              self.jump_to(start);
              if let Err(e) = self.parse_ascii_string() {
                return Err(e);
              }
              return Ok(())
            }else{
              return Err(ScannerError::InvalidNullLiteral);
            }
          }
        }else{
          return Err(ScannerError::UnexpectedTermination);
        }
      }
      self.add_token(TokenType::Null, Literals::Null, None, self.line);
      return Ok(());
    }else if first_character == 'f'{
      let data:[char;4] = ['a','l','s','e'];
      for d in data.into_iter(){
        if let Some(cache) = self.advance() {
          if cache != d{
            if is_ascii_from_chars(cache){
              self.jump_to(start);
              if let Err(e) = self.parse_ascii_string() {
                return Err(e);
              }
              return Ok(());
            }else{
              return Err(ScannerError::InvalidNullLiteral);
            }
          }
        }else{
          return Err(ScannerError::UnexpectedTermination);
        }
      }
      self.add_token(TokenType::Boolean, Literals::Boolean(false), None, self.line);
      return Ok(());
    }else if first_character == 't'{
      let data:[char;3] = ['r','u','e'];
      for d in data.into_iter(){
        if let Some(cache) = self.advance() {
          if cache != d{
            if is_ascii_from_chars(cache){
              self.jump_to(start);
              if let Err(e) = self.parse_ascii_string() {
                return Err(e);
              }
              return Ok(());
            }else{
              return Err(ScannerError::InvalidNullLiteral);
            }
          }
        }else{
          return Err(ScannerError::UnexpectedTermination);
        }
      }
      self.add_token(TokenType::Boolean, Literals::Boolean(true), None, self.line);
      return Ok(());
    }else{
      return Err(ScannerError::UnexpectedValue(first_character.to_string()));
    }
  }

  fn process_new_line(&mut self)->Result<(),ScannerError>{
    let current = self.current();
    if current == '\u{000a}'{
      self.add_line(1);
      return Ok(());
    }else if current == '\u{000d}'{
      if let Some(following) = self.advance() {
        if following == '\u{000a}' {
          self.add_line(1);
          return Ok(());
        } else {
          return Err(ScannerError::UnexpectedValue(r"\r".to_string()));
        }
      }else{
        return Err(ScannerError::UnexpectedTermination);
      }
    }else {
      return Err(ScannerError::UnexpectedValue(current.to_string()));
    }
  }

  fn current(&mut self)->char{
    if !self.is_at_end() {
      let data = self.data.chars().nth((self.pointer - 1) as usize).unwrap();

      data
    } else {
      '\0'
    }
  }

  fn is_at_end(&self) -> bool {
    self.pointer >= self.data.len() as i32
  }

  fn advance(&mut self) -> Option<char> {
    if !self.is_at_end() {
      let data = self.data.chars().nth(self.pointer as usize).unwrap();
      self.jump(1);

      Some(data)
    } else {
      None
    }
  }

  fn peek(&mut self) -> char {
    if !self.is_at_end() {
      let data = self.data.chars().nth(self.pointer as usize).unwrap();

      data
    } else {
      '\0'
    }
  }

  fn add_token(
    &mut self,
    token_type: TokenType,
    literal: Literals,
    lexeme: Option<String>,
    line: i32,
  ) -> () {
    self.vec.push(Token::new(token_type, literal, lexeme, line));
  }

  fn add_line(&mut self, count: i32) -> () {
    self.line += count;
  }

  fn jump(&mut self, count: i32) -> () {
    self.pointer += count;
  }

  fn get_pointer_position(&self) -> i32 {
    self.pointer
  }

  pub fn scan(&mut self) -> Result<&Vec<Token>, ScannerError> {
    self.scan_token()
  }
}