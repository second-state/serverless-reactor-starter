use wasm_bindgen::prelude::*;
use meval;

#[wasm_bindgen]
pub fn text_received(msg: String, _username: String, step_data: String) -> String {
  if msg == "#" {
    return format!(r#"{{"new_step": true}}"#);
  } else {
    let exp = match step_data == ""{
      true => msg,
      _ => format!("({}){}", step_data, msg)
    };
    let x = meval::eval_str(&exp).unwrap();
    return format!(r#"{{"result": "{}", "step": "{}"}}"#, x, exp);
  }
}
