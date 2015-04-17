extern crate tomson;
extern crate toml;

use std::io;

fn main() {
  match tomson::Json::as_toml(&mut io::stdin()) {
    Ok(ref t) =>
      println!("{}", toml::encode_str(t)),
    Err(e) =>
      panic!("invalid json: {:?}", e)
  }
}
