use indexmap::IndexMap;

use crate::value::Value;

pub type ObjectImpl = IndexMap<String, Value>;

pub type ArrayImpl = Vec<Value>;