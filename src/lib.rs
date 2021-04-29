use wasm_bindgen::prelude::*;
use serde_json::Value;
use meval;
use std::str;
use ssvm_process_interface::Command;

#[wasm_bindgen]
pub fn text_received(msg: String, _user_info: String, _step_data: String) -> String {
  let v: Vec<&str> = msg.split_whitespace().collect();
  let mut v1 = v[1].to_string();
  v1.make_ascii_uppercase();
  let mut v3 = v[3].to_string();
  v3.make_ascii_uppercase();
  let pair = v1 + "_" + &v3;

  let mut cmd = Command::new("http_proxy");
  // [currencyconverterapi key] comes from https://free.currencyconverterapi.com
  cmd.arg("get")
    .arg(format!("https://free.currconv.com/api/v7/convert?q={}&compact=ultra&apiKey=[currencyconverterapi key]", pair))
    .stdin_u8vec("".as_bytes());

  let out = cmd.output();
  if out.status != 0 {
      println!("Code: {}", out.status);
      println!("STDERR: {}", str::from_utf8(&out.stderr).unwrap());
      println!("STDOUT: {}", str::from_utf8(&out.stdout).unwrap());
      return str::from_utf8(&out.stderr).unwrap().to_string();
  }

  let cur = str::from_utf8(&out.stdout).unwrap();
  let cur: Value = serde_json::from_str(cur).unwrap();
  let cur = cur[pair].as_f64().unwrap();

  return format!(
    r#"{{"result": "{}", "step": "{}"}}"#,
    meval::eval_str(v[0].to_owned() + "*" + &cur.to_string()).unwrap(),
    ""
  );
}
