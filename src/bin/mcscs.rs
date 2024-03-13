/*
 * Copyright (c) 2024 Minecraft Server Config Script for Rust.
 */

use mcscs::pages::{clear_console, input, pause};
use mcscs::pages::{config, create, delete, init, start};

#[tokio::main]
async fn main() {
    init::main().await.expect("main()");
    loop {
        println!("1: 启动服务器");
        println!("2: 创建服务器");
        println!("3: 配置服务器");
        println!("4: 删除服务器");
        println!("0: 退出");
        print!("请选择一个选项: ");
        let input_value = input();
        if input_value == "0" {
            return;
        }
        clear_console();
        if input_value == "1" {
            start::main();
        } else if input_value == "2" {
            create::main().await;
        } else if input_value == "3" {
            config::main();
        } else if input_value == "4" {
            delete::main();
        } else {
            continue;
        }
        pause();
        clear_console();
    }
}
