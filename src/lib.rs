#![deny(missing_docs)]

//! tomson provides conversions from [Toml](http://alexcrichton.com/toml-rs) to [Json](https://doc.rust-lang.org/serialize/json/) and [Json](https://doc.rust-lang.org/serialize/json/) to [Toml](http://alexcrichton.com/toml-rs)
//!
//! # Example
//! 
//! ```
//! let toml = r#"
//! [foo]
//! bar = 1
//! "#;
//!
//! let json = r#"
//! {"foo":{"bar":1}}
//! "#;
//!
//! match tomson::Toml::as_json(&mut toml.to_string()) {
//!     Ok(json) => println!("json -> {:?}", json),
//!     Err(e)   => println!("invalid toml -> {:?}", e)
//! };
//!
//! match tomson::Json::as_toml(&mut json.to_string()) {
//!   Ok(toml) => println!("toml -> {:?}", toml),
//!   Err(e)   => println!("invalid json -> {:?}", e)
//! };
//! ```

extern crate toml;
extern crate rustc_serialize;

use rustc_serialize::json::{ self, ToJson };
use std::collections::BTreeMap;
use std::io::{ Read, Stdin };

/// Provides converstions from Json to Toml
pub struct Json;

/// Represents an Json input source
pub trait JsonSrc {
  /// attempt to parse source into Json value
  fn parse(&mut self) -> Result<json::Json, json::ParserError>;
}

macro_rules! to_json_src_impl_read {
  ($($t:ty), +) => (
    $(impl JsonSrc for $t {
      fn parse(&mut self) ->  Result<json::Json, json::ParserError> { 
        let mut src = String::new();
        let _ = self.read_to_string(&mut src);
        json::Json::from_str(&src)
      }
    })+ 
  )
}

to_json_src_impl_read! { Read, Stdin }

macro_rules! to_json_src_impl_json {
  ($($t:ty), +) => (
    $(impl JsonSrc for $t {
      fn parse(&mut self) -> Result<json::Json, json::ParserError> {
        Ok(self.to_json())
      }
    })+ 
  )
}

to_json_src_impl_json! { json::ToJson, json::Json }

impl JsonSrc for String {
  fn parse(&mut self) -> Result<json::Json, json::ParserError> {
    json::Json::from_str(self)
  }
}

impl Json {
  /// Convert Json to Toml
  pub fn as_toml<J: JsonSrc>(src: &mut J) -> Result<toml::Value, json::ParserError> {
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

macro_rules! to_toml_src_impl_read {
  ($($t:ty), +) => (
    $(impl TomlSrc for $t {
      fn parse(&mut self) -> Result<toml::Value, Vec<toml::ParserError>> {
        let mut src = String::new();
        let _ = self.read_to_string(&mut src);
        let mut parser = toml::Parser::new(&src);
        match parser.parse() {
          Some(value) => Ok(toml::Value::Table(value)),
          _           => Err(parser.errors)
        }
      }
    })+ 
  )
}

to_toml_src_impl_read! { Read, Stdin }

impl TomlSrc for String {
  fn parse(&mut self) -> Result<toml::Value, Vec<toml::ParserError>> {
    let mut parser = toml::Parser::new(self);
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

  #[test]
  fn test_to_json() {
    let res = Toml::as_json(&mut "[foo.bar]\n\nbaz=1".to_string());
    assert_eq!(res.is_ok(), true)
  }
  #[test]
  fn test_to_toml() {
   // let mut reader = BufReader::new(r#"{"foo":1}"#.as_bytes());
    //let mut src = r#"{"foo":1}"#.to_string();
    let res = Json::as_toml(&mut r#"{"foo":1}"#.to_string());
    assert_eq!(res.is_ok(), true)
  }
}
