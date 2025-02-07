use config::deserializer::{lexer::Lexer, Deserializer};

#[test]
fn test() {
  let d: String = String::from(
    r##"

name "Marquage";//comment

description "A simple mark language mainly used as config files";//comment

&year 2024;

version [1,0,0,"abc",true,[1,2,3,{"abc" 123;test [1,*year,3];}]];///////////a

authors {//comment
  "SuiBian9516" "m1311826090@outlook.com";//
}

is_latest false;

is_simple true;

is_widely_used void;

&variable "Only accept simple data structures";

copyright_year *year;"##,
  );
  let lex = Lexer::new(d);
  let mut parser = Deserializer::new(lex);
  match parser.parse() {
    Ok(t) => {
      println!("{:?}", t);
    },
    Err(e) => {
      panic!("Error Message:\n{:?}", e);
    },
  }
}
