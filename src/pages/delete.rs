/*
 * Copyright (c) 2024 Minecraft Server Config Script for Rust.
 */

use std::{env, fs};

use crate::pages::{choose_server, input};

/// 删除服务器页面
pub fn main() {
    let mut server = choose_server("需要删除");
    if server.is_null() {
        println!("你还没有创建任何一个服务器!");
        return;
    }
    loop {
        print!("确认删除?(Y/N) ");
        let input_value = input().to_lowercase();
        if input_value == "y" || input_value == "yes" {
            fs::remove_dir(
                env::current_dir()
                    .expect("main()")
                    .join("MCSCS")
                    .join("servers")
                    .join(server["name"].take().as_str().expect("main()")),
            )
            .expect("main()");
            break;
        }
        if input_value == "n" || input_value == "no" {
            break;
        }
        println!("输入错误,请重新输入!");
    }
}
