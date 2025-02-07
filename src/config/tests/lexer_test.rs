use config::deserializer::lexer::Lexer;
use config::deserializer::literal::Literal;
#[test]
fn string_test() {
  let d: String = String::from(
    r##"
  "String.test\nnewline\r\nnewline"  
"##,
  );
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::String("String.test\nnewline\r\nnewline".to_string()))
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn integer_number_test() {
  let d: String = String::from(
    r##"
  123456//comment
"##,
  );
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::UnsignedIntegerNumber(123456))
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn float_number_test() {
  let d: String = String::from(
    r##"
  123456.123456//comment
"##,
  );
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::FloatNumber(123456.123456))
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn negative_integer_number_test() {
  let d: String = String::from(
    r##"
  -123456
"##,
  );
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::SignedIntegerNumber(-123456))
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn negative_float_number_test() {
  let d: String = String::from(
    r##"
  -123456.123456
"##,
  );
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::FloatNumber(-123456.123456))
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn true_test() {
  let d: String = String::from(
    r##"
  true//comment
"##,
  );
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::Boolean(true))
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn false_test() {
  let d: String = String::from(
    r##"
  false//comment
"##,
  );
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::Boolean(false))
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn void_test() {
  let d: String = String::from(
    r##"
  void//comment
"##,
  );
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::Void)
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn brace_test() {
  let d: String = String::from(r##"{}"##);
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::OpenBrace)
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::CloseBrace)
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn bracket_test() {
  let d: String = String::from(r##"[]"##);
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::OpenBracket)
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::CloseBracket)
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn paren_test() {
  let d: String = String::from(r##"()"##);
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::OpenParen)
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::CloseParen)
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn pound_test() {
  let d: String = String::from(r##"#include "./abc";"##);
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::IncludeCommand("include".to_string(), "./abc".to_string()))
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn reference_test() {
  let d: String = String::from(r##"&abc;"##);
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::Reference("abc".to_string()))
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn dereference_test() {
  let d: String = String::from(r##"*abc;"##);
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::Dereference("abc".to_string()))
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn comma_test() {
  let d: String = String::from(r##","##);
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::Comma)
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn semicolon_test() {
  let d: String = String::from(r##";"##);
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::Semicolon)
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn comment_test() {
  let d: String = String::from(r##"//comment"##);
  let mut lex = Lexer::new(d);
  match lex.get() {
    Ok(data) => {
      assert_eq!(*data.get_literal_ref(), Literal::Comment)
    },
    Err(e) => {
      panic!("Error Message: \n{:?}", e);
    },
  }
}

#[test]
fn all_test() {
  let d: String = String::from(
    r##"#include "./path/to/what/you/want/to/include";//comment

name "Marquage";//comment

description "A simple mark language mainly used as config files";//comment

version [1,0,0];///////////a

authors {//comment
  "SuiBian9516" "m1311826090@outlook.com";//
};

is_latest false;

is_simple true;

is_widely_used void;

&variable "Only accept simple data structures";

&year 2024;

copyright_year *year;"##,
  );
  let mut lex = Lexer::new(d);
  loop {
    match lex.get() {
      Ok(t) => {
        if *t.get_literal_ref() == Literal::End {
          return;
        }
        println!("Received Token: {:?}", t);
      },
      Err(e) => {
        panic!("Error Message:\n{:?}", e);
      },
    }
  }
}
