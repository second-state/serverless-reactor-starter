use wasm_bindgen::prelude::*;
use ssvm_tensorflow_interface;
use urlencoding::encode;

#[wasm_bindgen]
pub fn text_received(_msg: String, _user_info: String, _step_data: String) -> String {
  "Please send a food picture (must be JPEG)".to_string()
}
#[wasm_bindgen]
pub fn image_received(img_buf: Vec<u8>, _image_key: String, _user_info: String, _step_data: String) -> String {
  let model_data: &[u8] = include_bytes!("lite-model_aiy_vision_classifier_food_V1_1.tflite");
  let labels = include_str!("aiy_food_V1_labelmap.txt");

  let flat_img = ssvm_tensorflow_interface::load_jpg_image_to_rgb8(&img_buf, 192, 192);

  let mut session = ssvm_tensorflow_interface::Session::new(&model_data, ssvm_tensorflow_interface::ModelType::TensorFlowLite);
  session.add_input("input", &flat_img, &[1, 192, 192, 3])
         .run();
  let res_vec: Vec<u8> = session.get_output("MobilenetV1/Predictions/Softmax");

  let mut i = 0;
  let mut max_index: i32 = -1;
  let mut max_value: u8 = 0;
  while i < res_vec.len() {
      let cur = res_vec[i];
      if cur > max_value {
          max_value = cur;
          max_index = i as i32;
      }
      i += 1;
  }

  let mut confidence = "possible";
  if max_value > 200 {
      confidence = "very likely";
  } else if max_value > 125 {
      confidence = "likely";
  } else if max_value > 50 {
      confidence = "possible";
  }

  let mut label_lines = labels.lines();
  for _i in 0..max_index {
    label_lines.next();
  }

  let class_name = label_lines.next().unwrap();

  if max_value > 50 {
    let s = format!(r#"
    {{
      "type": "post",
      "msg": [
        [{{
            "tag": "text",
            "text": "It is {} that the uploaded picture contains "
          }},
          {{
            "tag": "a",
            "href": "https://www.bing.com/search?q={}",
            "text": "{}"
          }}
        ]
      ]
    }}
    "#, confidence.to_string(), encode(class_name), class_name).replace("\n", "");
    return format!(r#"{{"result": {}}}"#, s);
  } else {
    return format!("No food detected in the uploaded picture");
  }
}

