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

<a href="https://github.com/second-state/serverless-reactor-starter/fork">请 fork 这个代码仓库</a>。默认的函数是一个修改图片尺寸的机器人，向它发送一张图片和一个宽度值，它就会返回修改尺寸后的图片。

```rust
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

