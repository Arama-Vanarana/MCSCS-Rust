// use crate::library::pages::{clear_console, config, create, delete, init, input, start};

mod pages;
use crate::pages::{clear_console, input};
use crate::pages::{config, create, delete, init, start};

pub(crate) mod aria2c;
pub(crate) mod fastmirror;
pub(crate) mod java;
pub(crate) mod server;

#[tokio::main]
async fn main() {
    if let Err(err) = init::main().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    loop {
        println!("1: 启动服务器");
        println!("2: 创建服务器");
        println!("3: 配置服务器");
        println!("4: 删除服务器");
        println!("0: 退出");
        print!("请选择一个选项: ");
        let input_value = input();
        if input_value == "1" {
            start::main();
        } else if input_value == "2" {
            create::main().await;
        } else if input_value == "3" {
            config::main();
        } else if input_value == "4" {
            delete::main();
        } else if input_value == "0" {
            return;
        }
        clear_console();
    }
}

#[cfg(test)]
mod tests {
    mod downloads;
    mod java;
}
