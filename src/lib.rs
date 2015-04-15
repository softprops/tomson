extern crate toml;
extern crate rustc_serialize;

use rustc_serialize::json::{ self, ToJson };
use std::collections::BTreeMap;
use std::io::Read;

pub struct Json;

impl Json {
  /// convert Json to Toml
  pub fn as_toml(read: &mut Read) -> Option<toml::Value> {
    let mut src = String::new();
    let _ = read.read_to_string(&mut src);
    let json = json::Json::from_str(&src);
    fn json_as_toml(value: &json::Json) -> toml::Value {
      match *value {
        json::Json::I64(ref v)     => toml::Value::Integer(v.clone()),
        json::Json::U64(ref v)     => toml::Value::Integer(v.clone() as i64),
        json::Json::F64(ref v)     => toml::Value::Float(v.clone()),
        json::Json::String(ref v)  => toml::Value::String(v.clone()),
        json::Json::Boolean(ref v) => toml::Value::Boolean(v.clone()),
        json::Json::Array(ref v)   => {
          let mut tl = Vec::<toml::Value>::new();
          for jv in v.iter() {
            tl.push(json_as_toml(jv));
          }
          toml::Value::Array(tl)
        },
        json::Json::Object(ref v)  => {
          let mut tm = BTreeMap::new();
          for (k,v) in v.iter() {
            tm.insert(k.clone(), json_as_toml(v));
          }
          toml::Value::Table(tm)
        },
        json::Json::Null           => toml::Value::String("".to_string())
      }
    }
    json.ok().map(|value| json_as_toml(&value))
  }
}

pub struct Toml;

impl Toml {
  /// Convert Toml to Json
  pub fn as_json(read: &mut Read) -> Option<json::Json> {
    let mut src = String::new();
    let _ = read.read_to_string(&mut src);
    let mut parser = toml::Parser::new(&src);

    fn value_as_json(toml: &toml::Value) -> json::Json {
      match *toml {
        toml::Value::Table(ref value)    => table_as_json(value),       
        toml::Value::Array(ref array)    => {
          let mut vec = Vec::new();
          for value in array.iter() {
            vec.push(value_as_json(value));
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

    fn table_as_json(table: &toml::Table) -> json::Json {
      let mut map = BTreeMap::new();
      for (k,v) in table.iter() {
        map.insert(k.to_string(), value_as_json(v));
      };
      map.to_json()
    }

    parser.parse().map(|tbl| table_as_json(&tbl)) 
  }
}
