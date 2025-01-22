
use config::deserializer::lexer::Lexer;
use config::Value;

#[test]
fn string_test(){
  let d:String = String::from(r##"
  "String.test\nnewline\r\nnewline"  
"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::String("String.test\nnewline\r\nnewline".to_string()))
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn integer_number_test(){
  let d:String = String::from(r##"
  123456//comment
"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::IntegerNumber(123456))
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn float_number_test(){
  let d:String = String::from(r##"
  123456.123456//comment
"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::FloatNumber(123456.123456))
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn negative_integer_number_test(){
  let d:String = String::from(r##"
  -123456
"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::IntegerNumber(-123456))
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn negative_float_number_test(){
  let d:String = String::from(r##"
  -123456.123456
"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::FloatNumber(-123456.123456))
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}


#[test]
fn true_test(){
  let d:String = String::from(r##"
  true//comment
"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::Boolean(true))
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn false_test(){
  let d:String = String::from(r##"
  false//comment
"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::Boolean(false))
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn void_test(){
  let d:String = String::from(r##"
  void//comment
"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::Void)
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn brace_test(){
  let d:String = String::from(r##"{}"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::OpenBrace)
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::CloseBrace)
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn bracket_test(){
  let d:String = String::from(r##"[]"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::OpenBracket)
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::CloseBracket)
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn paren_test(){
  let d:String = String::from(r##"()"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::OpenParen)
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::CloseParen)
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn pound_test(){
  let d:String = String::from(r##"#"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::Pound)
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn reference_test(){
  let d:String = String::from(r##"&"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::Reference)
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn dereference_test(){
  let d:String = String::from(r##"*"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::Dereference)
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn comma_test(){
  let d:String = String::from(r##","##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::Comma)
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn semicolon_test(){
  let d:String = String::from(r##";"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::Semicolon)
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}

#[test]
fn comment_test(){
  let d:String = String::from(r##"//comment"##);
  let mut lex = Lexer::new(d);
  match lex.get(){
    Ok(data)=>{
      assert_eq!(*data.get_value(),Value::Comment)
    }
    Err(e)=>{
      panic!("Error Message: \n{:?}", e);
    }
  }
}