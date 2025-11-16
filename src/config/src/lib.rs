//! Config Library have the ability to handle dedicated config file format.
//! It offers serialization and deserialization methods to operate config file.
//!
//! # Structure of Marquage
//! ```marquage
//! name "Marquage";
//!
//! description "A simple mark language mainly used as config files";
//!
//! version [0,1,0];
//!
//! authors {
//!   "SuiBian9516" "m1311826090@outlook.com";
//! }
//!
//! birth 2024;
//! ```
//!
//! The structure of Marquage seems like JSON, but it use whitespace as separator.
//!
mod config;
mod deserializable;
pub mod deserializer;
pub mod map;
mod serializable;
pub mod serializer;
pub mod value;
#[macro_use]
mod macros;

pub use config::Config;
pub use deserializable::{Deserializable, DeserializableError};
pub use serializable::Serializable;
