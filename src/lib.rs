#![deny(missing_docs)]

//! tomson provides conversions from [Toml](http://alexcrichton.com/toml-rs) to [Json](https://doc.rust-lang.org/serialize/json/) and [Json](https://doc.rust-lang.org/serialize/json/) to [Toml](http://alexcrichton.com/toml-rs)

extern crate toml;
extern crate rustc_serialize;

use rustc_serialize::json::{ self, ToJson };
use std::collections::BTreeMap;
use std::io::Read;

/// Provides converstions from Json to Toml
pub struct Json;

impl Json {
  /// Convert Json to Toml
  pub fn as_toml(read: &mut Read) -> Result<toml::Value, json::ParserError> {
    let mut src = String::new();
    let _ = read.read_to_string(&mut src);
    let json = json::Json::from_str(&src);
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
    json.map(|value| adapt(&value))
  }
}

/// Provides convertions from Toml to Json
pub struct Toml;

impl Toml {
  /// Convert Toml to Json
  pub fn as_json(read: &mut Read) -> Result<json::Json, Vec<toml::ParserError>> {
    let mut src = String::new();
    let _ = read.read_to_string(&mut src);
    let mut parser = toml::Parser::new(&src);

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

    match parser.parse() {
      Some(value) => Ok(adapt(&toml::Value::Table(value))),
      _ => Err(parser.errors)
    }
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
    assert_eq!(res.is_some(), true)
  }
  #[test]
  fn test_to_toml() {
    let mut reader = BufReader::new(r#"{"foo":1}"#.as_bytes());
    let res = Json::as_toml(&mut reader);
    assert_eq!(res.is_some(), true)
  }
}
