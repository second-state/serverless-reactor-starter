Serverless Reactors 让你可以快速零成本地构建发布飞书机器人。

# 安装必要的软件

请确保您已经安装了[Rust](https://www.rust-lang.org/tools/install), 和[ssvmup](https://www.secondstate.io/articles/ssvmup/).
为了您的方便，下面是 Linux / Mac / PowerShell 的安装命令。

```src
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env
$ rustup override set 1.50.0

$ curl https://raw.githubusercontent.com/second-state/ssvmup/master/installer/init.sh -sSf | sh
```

# 编写机器人逻辑

<a href="https://github.com/second-state/serverless-reactor-starter/fork">请 fork 这个代码仓库</a>。默认的函数是一个汇率换算机器人，向它发一个“1 usd to cny”的消息，它就会回答 1 美元当前折合多少人民币，比如 "6.470397"。

```rust
use wasm_bindgen::prelude::*;
use serde_json::Value;
use meval;
use std::str;
use ssvm_process_interface::Command;

#[wasm_bindgen]
pub fn text_received(msg: String, _username: String, _step_data: String) -> String {
  let v: Vec<&str> = msg.split_whitespace().collect();
  let mut v1 = v[1].to_string();
  v1.make_ascii_uppercase();
  let mut v3 = v[3].to_string();
  v3.make_ascii_uppercase();
  let pair = v1 + "_" + &v3;
  let mut cmd = Command::new("http_proxy");
  cmd.arg("get")
    .arg(format!("https://free.currconv.com/api/v7/convert?q={}&compact=ultra&apiKey=975724bc7bef91dd5877", pair))
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
```

改动 <a href="src/lib.rs">src/lib.rs</a> 这个文件，将它改为你的机器人逻辑。具体函数的写法请<a href="http://reactor.secondstate.info/docs/ch01-00-wasm-functions.html">参见文档</a>。

# 编译

将机器人的函数编译成可以部署的 WebAssembly 文件。

```src
$ ssvmup build
```

# 部署

<a href="http://reactor.secondstate.info/docs/user-create-a-bot.html">参见文档</a>创建一个飞书企业应用与 <a href="http://reactor.secondstate.info/">Serverless Reactor</a> 的对应 app，将编译成功的 `pkg/converter_robot_lib.wasm` 文件上传到 Serverless Reactor，并把生成的 service URL 提交给飞书。

祝贺你！你现在有了一个属于你的飞书机器人！

