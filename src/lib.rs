use rand::Rng;
use serde_json::Value;
use ssvm_process_interface::Command;
use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn text_received(msg: String, _user_info: String, _step_data: String) -> String {
  let mut rng = rand::thread_rng();
  let page_number = rng.gen_range(0..6198);
  let mut cmd = Command::new("http_proxy");
  cmd.arg("get").arg(format!(
    "https://api.artic.edu/api/v1/artworks?fields=id,title,artist_display,image_id&limit=1&page={}",
    page_number
  ));
  let out = cmd.output();
  if out.status != 0 {
    println!("Code: {}", out.status);
    println!("STDERR: {}", str::from_utf8(&out.stderr).unwrap());
    println!("STDOUT: {}", str::from_utf8(&out.stdout).unwrap());
    return str::from_utf8(&out.stderr).unwrap().to_string();
  }

  let artwork = str::from_utf8(&out.stdout).unwrap();
  let artwork: Value = serde_json::from_str(artwork).unwrap();

  // if image-id is null
  if artwork["data"][0]["image_id"] == Value::Null {
    return String::from("No image_id available. Please try again.");
  }

  let title = artwork["data"][0]["title"].as_str().unwrap();
  let artist = artwork["data"][0]["artist_display"].as_str().unwrap();
  let image_url = format!(
    "https://www.artic.edu/iiif/2/{}/full/843,/0/default.jpg",
    artwork["data"][0]["image_id"].as_str().unwrap()
  );

  return format!(
    "*Art title*: {}\n*Artist*: {}\n*Art image URL*: <{}>",
    title, artist.replace("\n", ", "), image_url
  );
}
