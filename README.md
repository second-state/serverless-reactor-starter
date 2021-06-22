Serverless Reactors 让你可以快速零成本地构建发布飞书机器人。

# 安装必要的软件

请确保您已经安装了[Rust](https://www.rust-lang.org/tools/install), 和[rustwasmc](https://www.secondstate.io/articles/rustwasmc/).
为了您的方便，下面是 Linux / Mac / PowerShell 的安装命令。

```src
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env
$ rustup override set 1.50.0

$ curl https://raw.githubusercontent.com/second-state/rustwasmc/master/installer/init.sh -sSf | sh
```

# 编写机器人逻辑

<a href="https://github.com/second-state/serverless-reactor-starter/fork">请 fork 这个代码仓库</a>。默认的函数是一个 AI 推理机器人，向它发一张包含食物的照片，它就会回答识别出的食物名称。

```rust
use wasm_bindgen::prelude::*;
use ssvm_tensorflow_interface;

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
    return format!("It is {} that the uploaded picture contains {}." , confidence.to_string(), class_name);
  } else {
    return format!("No food detected in the uploaded picture");
  }
}
```

改动 <a href="src/lib.rs">src/lib.rs</a> 这个文件，将它改为你的机器人逻辑。具体函数的写法请<a href="http://reactor.secondstate.info/docs/ch01-00-wasm-functions.html">参见文档</a>。

# 编译

将机器人的函数编译成可以部署的 WebAssembly 文件。

```src
$ rustwasmc build
```

# 部署

<a href="http://reactor.secondstate.info/docs/user-create-a-bot.html">参见文档</a>创建一个飞书企业应用与 <a href="http://reactor.secondstate.info/">Serverless Reactor</a> 的对应 app，将编译成功的 `pkg/calculator_lib_bg.wasm` 文件上传到 Serverless Reactor，并把生成的 service URL 提交给飞书。

祝贺你！你现在有了一个属于你的飞书机器人！

