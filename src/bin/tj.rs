extern crate tomson;
extern crate rustc_serialize;

use rustc_serialize::json;
use std::io;

fn main() {
  match tomson::Toml::as_json(&mut io::stdin()) {
    Ok(ref json) =>
      println!("{}", json::as_json(json)),
    Err(errs) =>      
      panic!("invalid toml: {:?}", errs)
  }
}
