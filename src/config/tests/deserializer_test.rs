use config::{
  deserializer::{lexer::Lexer, Deserializer},
  Value,
};

fn match_value_with_panics(data: &Value, matching: Value) {
  if data != &matching {
    panic!("Matching failed when comparing {:?} with {:?}", data, matching)
  }
}

macro_rules! create_string {
  ($data:expr) => {
    String::from($data)
  };
}

#[test]
fn test_simple_case() {
  let data = String::from(
    r###"
  test_index 1;
  test_name "The simplest test case";

  test_string "This is string";
  test_number {
    integer 2025;
    float 3.14;
    negative_integer -2025;
    negative_float -3.14;
  }
  test_boolean {
    true_case true;
    false_case false;
  }
  test_void void;
  "###,
  );
  let lexer = Lexer::new(data);
  let instance = Deserializer::new(lexer);
  match instance.parse() {
    Ok(cache) => {
      match_value_with_panics(&cache["test_index"], Value::UnsignedIntegerNumber(1));
      match_value_with_panics(&cache["test_name"], Value::String(create_string!("The simplest test case")));
      match_value_with_panics(&cache["test_string"], Value::String(create_string!("This is string")));
      match_value_with_panics(&cache["test_number"]["integer"], Value::UnsignedIntegerNumber(2025));
      match_value_with_panics(&cache["test_number"]["float"], Value::FloatNumber(3.14));
      match_value_with_panics(&cache["test_number"]["negative_integer"], Value::SignedIntegerNumber(-2025));
      match_value_with_panics(&cache["test_number"]["negative_float"], Value::FloatNumber(-3.14));
      match_value_with_panics(&cache["test_boolean"]["true_case"], Value::Boolean(true));
      match_value_with_panics(&cache["test_boolean"]["false_case"], Value::Boolean(false));
      match_value_with_panics(&cache["test_void"], Value::Void);
    },
    Err(e) => {
      panic!("{}", e);
    },
  }
}

#[test]
fn test_object_case() {
  let data = String::from(
    r###"
  test_index 2;
  test_name "The object test case";

  test_object {
    test_object_state true;

    test_object {
      layer 2;

      test_object {
        layer 3;

        test_object {
          layer 4;
        }
      }
    }
  }
  "###,
  );
  let lexer = Lexer::new(data);
  let instance = Deserializer::new(lexer);
  match instance.parse() {
    Ok(cache) => {
      match_value_with_panics(&cache["test_index"], Value::UnsignedIntegerNumber(2));
      match_value_with_panics(&cache["test_name"], Value::String(create_string!("The object test case")));
      match_value_with_panics(&cache["test_object"]["test_object_state"], Value::Boolean(true));
      match_value_with_panics(&cache["test_object"]["test_object"]["layer"], Value::UnsignedIntegerNumber(2));
      match_value_with_panics(&cache["test_object"]["test_object"]["test_object"]["layer"], Value::UnsignedIntegerNumber(3));
      match_value_with_panics(&cache["test_object"]["test_object"]["test_object"]["test_object"]["layer"], Value::UnsignedIntegerNumber(4));
    },
    Err(e) => {
      panic!("{}", e);
    },
  }
}

#[test]
fn test_array_case() {
  let data = String::from(
    r###"
  test_index 3;
  test_name "The array test case";

  test_array [1,2,3];

  test_multi_array [['multi_array']];
  "###,
  );
  let lexer = Lexer::new(data);
  let instance = Deserializer::new(lexer);
  match instance.parse() {
    Ok(cache) => {
      match_value_with_panics(&cache["test_index"], Value::UnsignedIntegerNumber(3));
      match_value_with_panics(&cache["test_name"], Value::String(create_string!("The array test case")));
      match_value_with_panics(&cache["test_array"], Value::Array(vec![Value::UnsignedIntegerNumber(1), Value::UnsignedIntegerNumber(2), Value::UnsignedIntegerNumber(3)]));
      match_value_with_panics(&cache["test_multi_array"][0], Value::Array(vec![Value::String(create_string!("multi_array"))]));
    },
    Err(e) => {
      panic!("{}", e);
    },
  }
}

#[test]
fn test_variable_case() {
  let data = String::from(
    r###"
  test_index 4;
  test_name "The variable test case";

  &simple_data_type 'test';

  using_simple_data_type *simple_data_type;

  &complex_data_type {
    test "test";
  }

  using_complex_data_type *complex_data_type;
  "###,
  );
  let lexer = Lexer::new(data);
  let instance = Deserializer::new(lexer);
  match instance.parse() {
    Ok(cache) => {
      match_value_with_panics(&cache["test_index"], Value::UnsignedIntegerNumber(4));
      match_value_with_panics(&cache["test_name"], Value::String(create_string!("The variable test case")));
      match_value_with_panics(&cache["using_simple_data_type"], Value::String(create_string!("test")));
      match_value_with_panics(&cache["using_complex_data_type"]["test"], Value::String(create_string!("test")));
    },
    Err(e) => {
      panic!("{}", e);
    },
  }
}

#[test]
fn comprehensive_test_case() {
  let data = String::from(
    r###"
  test_index 5;
  test_name "The comprehensive test case";

  name "Marquage";

  description "A simple mark language mainly used as config files";

  version [1,0,0];

  authors {
    "SuiBian9516" "m1311826090@outlook.com";
  }

  is_latest false;

  is_simple true;

  is_widely_used void;

  &year 2024;

  copyright_year *year;
  "###,
  );
  let lexer = Lexer::new(data);
  let instance = Deserializer::new(lexer);
  match instance.parse() {
    Ok(cache) => {
      match_value_with_panics(&cache["test_index"], Value::UnsignedIntegerNumber(5));
      match_value_with_panics(&cache["test_name"], Value::String(create_string!("The comprehensive test case")));
      match_value_with_panics(&cache["version"], Value::Array(vec![Value::UnsignedIntegerNumber(1), Value::UnsignedIntegerNumber(0), Value::UnsignedIntegerNumber(0)]));
      match_value_with_panics(&cache["authors"]["SuiBian9516"], Value::String(create_string!("m1311826090@outlook.com")));
      match_value_with_panics(&cache["is_latest"], Value::Boolean(false));
      match_value_with_panics(&cache["is_simple"], Value::Boolean(true));
      match_value_with_panics(&cache["is_widely_used"], Value::Void);
      match_value_with_panics(&cache["copyright_year"], Value::UnsignedIntegerNumber(2024));
    },
    Err(e) => {
      panic!("{}", e);
    },
  }
}
