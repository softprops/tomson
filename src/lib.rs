extern crate toml;
extern crate rustc_serialize;

use std::collections::BTreeMap;
use std::io::Read;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
pub struct Toml;
impl Toml {
  
  pub fn as_json(read: &mut Read) -> Option<Json> {    
    let mut src = String::new();
    let _ = read.read_to_string(&mut src);
    let mut parser = toml::Parser::new(&src);

    fn value_as_json(toml: &toml::Value) -> Json {
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

    fn table_as_json(table: &toml::Table) -> Json {
      let mut map = BTreeMap::new();
      for (k,v) in table.iter() {
        map.insert(k.to_string(), value_as_json(v));
      };
      map.to_json()
    }

    parser.parse().map(|tbl| table_as_json(&tbl)) 
  }

}

