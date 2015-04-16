extern crate tomson;
extern crate toml;

use std::io;

fn main() {
  match tomson::Json::as_toml(&mut io::stdin()) {
    Some(ref t) =>
      println!("{}", toml::encode_str(t)),
    _ =>
      panic!("invalid toml")
  }
}
