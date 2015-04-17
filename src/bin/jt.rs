extern crate tomson;
extern crate toml;

use std::io;

fn main() {
  match tomson::Json::Read(Box::new(io::stdin())).as_toml() {
    Ok(ref t) =>
      println!("{}", toml::encode_str(t)),
    Err(e) =>
      panic!("invalid json: {:?}", e)
  }
}
