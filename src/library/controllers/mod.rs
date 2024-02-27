use std::io::{self, Write};

pub mod aria2c;
// Aria2下载引擎
pub mod fastmirror;
// FastMirror API
pub mod java;
// Java环境寻找
pub mod server; // 获取/更改服务器配置

#[doc = "返回输入的内容"]
pub fn input() -> String {
    io::stdout().flush().expect("无法刷新stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取 stdin 失败");
    input = input.trim().to_string();
    input
}
