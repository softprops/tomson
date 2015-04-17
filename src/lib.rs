#![deny(missing_docs)]

//! tomson provides conversions from [Toml](http://alexcrichton.com/toml-rs) to [Json](https://doc.rust-lang.org/serialize/json/) and [Json](https://doc.rust-lang.org/serialize/json/) to [Toml](http://alexcrichton.com/toml-rs)
//!
//! # Example
//! 
//! ```
//! use std::io::BufReader;
//!
//! let toml = r#"
//! [foo]
//! bar = 1
//! "#;
//!
//! let json = r#"
//! {"foo":{"bar":1}}
//! "#;
//!
//! match tomson::Toml::as_json(&mut BufReader::new(toml.as_bytes())) {
//!     Ok(json) => println!("json -> {:?}", json),
//!     Err(e)   => println!("invalid toml -> {:?}", e)
//! };
//!
//! match tomson::Json::as_toml(&mut BufReader::new(json.as_bytes())) {
//!   Ok(toml) => println!("toml -> {:?}", toml),
//!   Err(e)   => println!("invalid json -> {:?}", e)
//! };
//! ```

extern crate toml;
extern crate rustc_serialize;

use rustc_serialize::json::{ self, ToJson };
use std::collections::BTreeMap;
use std::io;

/// Provides converstions from Json to Toml
pub struct Json;

/// Represents an Json input source
pub trait JsonSrc {
  /// attempt to parse source into Json value
  fn parse(&mut self) -> Result<json::Json, json::ParserError>;
}

impl<R: io::Read> JsonSrc for R {
  fn parse(&mut self) -> Result<json::Json, json::ParserError> {
    let mut src = String::new();
    let _ = self.read_to_string(&mut src);
    json::Json::from_str(&src)
  }
}

impl Json {
  /// Convert Json to Toml
  pub fn as_toml(src: &mut JsonSrc) -> Result<toml::Value, json::ParserError> {
    fn adapt(value: &json::Json) -> toml::Value {
      match *value {
        json::Json::I64(ref v)     => toml::Value::Integer(v.clone()),
        json::Json::U64(ref v)     => toml::Value::Integer(v.clone() as i64),
        json::Json::F64(ref v)     => toml::Value::Float(v.clone()),
        json::Json::String(ref v)  => toml::Value::String(v.clone()),
        json::Json::Boolean(ref v) => toml::Value::Boolean(v.clone()),
        json::Json::Array(ref v)   => {
          let mut tl = Vec::<toml::Value>::new();
          for jv in v.iter() {
            tl.push(adapt(jv));
          }
          toml::Value::Array(tl)
        },
        json::Json::Object(ref v)  => {
          let mut tm = BTreeMap::new();
          for (k,v) in v.iter() {
            tm.insert(k.clone(), adapt(v));
          }
          toml::Value::Table(tm)
        },
        json::Json::Null           => toml::Value::String("".to_string())
      }
    }
    src.parse().map(|value| adapt(&value))
  }
}

/// Provides convertions from Toml to Json
pub struct Toml;

/// Represents an Toml input source
pub trait TomlSrc {
  /// attempt to parse source into Toml value
  fn parse(&mut self) -> Result<toml::Value, Vec<toml::ParserError>>;
}

impl<R: io::Read> TomlSrc for R {
  fn parse(&mut self) -> Result<toml::Value, Vec<toml::ParserError>> {
    let mut src = String::new();
    let _ = self.read_to_string(&mut src);
    let mut parser = toml::Parser::new(&src);
    match parser.parse() {
      Some(value) => Ok(toml::Value::Table(value)),
      _           => Err(parser.errors)
    }
  }
}

impl Toml {
  
  /// Convert Toml to Json
  pub fn as_json(src: &mut TomlSrc) -> Result<json::Json, Vec<toml::ParserError>> {
    fn adapt(toml: &toml::Value) -> json::Json {
      match *toml {
        toml::Value::Table(ref value)    => {
          let mut map = BTreeMap::new();
          for (k,v) in value.iter() {
            map.insert(k.to_string(), adapt(v));
          };
          map.to_json()
        },
        toml::Value::Array(ref array)    => {
          let mut vec = Vec::new();
          for value in array.iter() {
            vec.push(adapt(value));
          };
          vec.to_json()
        },
        toml::Value::String(ref value)   => value.to_json(),
        toml::Value::Integer(ref value)  => value.to_json(),
        toml::Value::Float(ref value)    => value.to_json(),
        toml::Value::Boolean(ref value)  => value.to_json(),
        toml::Value::Datetime(ref value) => value.to_json()
      }
    }

    src.parse().map(|value| adapt(&value))
  }
}

#[cfg(test)]
mod tests {
  use super::{ Json, Toml };
  use std::io::BufReader;
  #[test]
  fn test_to_json() {
    let mut reader = BufReader::new("[foo.bar]\n\nbaz=1".as_bytes());
    let res = Toml::as_json(&mut reader);
    assert_eq!(res.is_ok(), true)
  }
  #[test]
  fn test_to_toml() {
    let mut reader = BufReader::new(r#"{"foo":1}"#.as_bytes());
    let res = Json::as_toml(&mut reader);
    assert_eq!(res.is_ok(), true)
  }
}
