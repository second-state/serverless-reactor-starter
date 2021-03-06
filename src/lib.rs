use wasm_bindgen::prelude::*;
use meval;

#[wasm_bindgen]
pub fn text_received(msg: String, _user_info: String, _step_data: String) -> String {
  let x = meval::eval_str(&msg).unwrap();
  return format!("{}", x);
}
