extern crate tomson;
extern crate rustc_serialize;

use rustc_serialize::json;
use std::io;

fn main() {
  match tomson::Toml::Read(Box::new(io::stdin())).as_json() {
    Ok(ref json) =>
      println!("{}", json::as_json(json)),
    Err(errs) =>      
      panic!("invalid toml: {:?}", errs)
  }
}
