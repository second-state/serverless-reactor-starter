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

<a href="https://github.com/second-state/serverless-reactor-starter/fork">请 fork 这个代码仓库</a>。默认的函数是一个计算器机器人，向它发一个“2+2”的消息，它就会回答“4”。

```rust
use wasm_bindgen::prelude::*;
use meval;

#[wasm_bindgen]
pub fn text_received(msg: String, _username: String, _step_data: String) -> String {
  let x = meval::eval_str(&msg).unwrap();
  return format!("{}", x);
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

