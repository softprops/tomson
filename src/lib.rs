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
//! match tomson::Toml::Read(Box::new(BufReader::new(toml.as_bytes()))).as_json() {
//!     Ok(json) => println!("json -> {:?}", json),
//!     Err(e)   => println!("invalid toml -> {:?}", e)
//! };
//!
//! match tomson::Json::Read(Box::new(BufReader::new(json.as_bytes()))).as_toml() {
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
pub enum Json {
  /// a Read instance for Json
  Read(Box<io::Read>)
}

impl Json {
  fn parse(&mut self) -> Result<json::Json, json::ParserError> {
    match *self {
      Json::Read(ref mut r) => {
        let mut src = String::new();
        let _ = r.read_to_string(&mut src);
        json::Json::from_str(&src)
      }
    }
  }
  /// Convert Json to Toml
  pub fn as_toml(&mut self) -> Result<toml::Value, json::ParserError> {
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
    self.parse().map(|value| adapt(&value))
  }
}

/// Provides convertions from Toml to Json
pub enum Toml {
  /// A Read instance for Toml
  Read(Box<io::Read>)
}

impl Toml {
  fn parse(&mut self) -> Result<toml::Value, Vec<toml::ParserError>> {
    match *self {
      Toml::Read(ref mut r) => {
        let mut src = String::new();
        let _ = r.read_to_string(&mut src);
        let mut parser = toml::Parser::new(&src);
        match parser.parse() {
          Some(value) => Ok(toml::Value::Table(value)),
          _           => Err(parser.errors)
        }
      }
    }
  }
  /// Convert Toml to Json
  pub fn as_json(&mut self) -> Result<json::Json, Vec<toml::ParserError>> {
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

    self.parse().map(|value| adapt(&value))
  }
}

#[cfg(test)]
mod tests {
  use super::{ Json, Toml };
  use std::io::BufReader;
  #[test]
  fn test_to_json() {
    let reader = Box::new(BufReader::new("[foo.bar]\n\nbaz=1".as_bytes()));
    let res = Toml::Read(reader).as_json();
    assert_eq!(res.is_ok(), true)
  }
  #[test]
  fn test_to_toml() {
    let reader = Box::new(BufReader::new(r#"{"foo":1}"#.as_bytes()));
    let res = Json::Read(reader).as_toml();
    assert_eq!(res.is_ok(), true)
  }
}
