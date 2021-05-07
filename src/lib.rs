use wasm_bindgen::prelude::*;
use serde_json::{Value};

#[wasm_bindgen]
pub fn text_received(msg: String, _user_info: String, _step_data: String) -> String {
  let obj: Value = match serde_json::from_str(&msg) {
    Ok(v) => v,
    Err(e) => return format!("{:?}", e)
  };
  let str = serde_json::to_string_pretty(&obj).unwrap();
  return format!(r#"{{"result": "{}"}}"#, str.replace("\"", "\\\""));
}

