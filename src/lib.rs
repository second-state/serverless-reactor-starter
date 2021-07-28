use rand::Rng;
use serde_json::Value;
use std::str;
use wasm_bindgen::prelude::*;
// for offline test:
// use wasmedge_process_interface::Command;
use ssvm_process_interface::Command;

#[wasm_bindgen]
pub fn text_received(msg: String, _user_info: String, _step_data: String) -> String {
  let mut rng = rand::thread_rng();
  let page_number = rng.gen_range(0..6198);
  let mut cmd = Command::new("http_proxy");
  cmd
    .arg("get")
    .arg(format!(
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

  // println!("{}", artwork);

  // TODO: if image_id is null --> request again
  if artwork["data"][0]["image_id"] == Value::Null {
    return String::from("No image_id available. Please try again.");
  }

  let image_url = format!(
    "https://www.artic.edu/iiif/2/{}/full/843,/0/default.jpg",
    artwork["data"][0]["image_id"].as_str().unwrap()
  );

  return format!("The artwork URL is <{}>", image_url);
}
