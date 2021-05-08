use wasm_bindgen::prelude::*;
use serde_json::{json, Value};
use image;
use image::{GenericImageView};

#[wasm_bindgen]
pub fn text_received(msg: String, _user_info: String, step_data: String) -> String {
  let mut last_step_data = match step_data.len() > 0 {
    true => serde_json::from_str(step_data.as_str()).unwrap(),
    false => return String::from("请先发送一张图片")
  };

  let width: u32 = match msg.parse().ok() {
    Some(width) => width,
    None => return String::from("请回复数字")
  };
  extend_step_data(&mut last_step_data, vec!(
    (String::from("width"), json!(width))
  ));
  let step = Some(format!("{}", last_step_data));

  let next = Some(vec!(
    json!({"type": "image", "require_image": last_step_data["pic"]})
  ));

  return format_resp(next, None, None, step, false);
}

#[wasm_bindgen]
pub fn image_received(_img_buf: Vec<u8>, image_key: String, _user_info: String, _step_data: String) -> String {
  let step_data = json!({
    "pic": image_key
  });
  let step = Some(format!("{}", step_data));
  let resp_msg = Some(vec!(Value::String(String::from("你想把图片改成多宽？"))));
  format_resp(None, resp_msg, None, step, true)
}

#[wasm_bindgen]
pub fn expect_image(img_buf: Vec<u8>, _user_info: String, step_data: String, _resp_index: i32) -> Vec<u8> {
  let last_step_data: Value = serde_json::from_str(step_data.as_str()).unwrap();
  let x = resize_pic(
    &img_buf,
    last_step_data["width"].as_u64().unwrap() as u32,
  );
  return x;
}

fn format_resp(next: Option<Vec<Value>>, result: Option<Vec<Value>>, receiver: Option<&str>, step: Option<String>, new_step: bool) -> String {
  let resp = json!({
    "next": next,
    "result": result,
    "receiver": receiver,
    "step": step,
    "new_step": new_step
  });

  return format!("{}", resp);
}

fn extend_step_data(step_data: &mut Value, items: Vec<(String, Value)>) {
  let m = step_data.as_object_mut().unwrap();
  m.extend(items);
}

fn resize_pic(img_buf: &[u8], width: u32) -> Vec<u8> {
  let img = image::load_from_memory(img_buf).unwrap();
  let height = ((img.height() as f32 / img.width() as f32) * width as f32) as u32 + 100;
  let img = img.resize(width, height, image::imageops::FilterType::Triangle);
  let mut buf = vec![];
  img.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();
  return buf;
}

