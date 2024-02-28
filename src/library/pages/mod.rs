use std::io::{self, Write};

use console::Term;
use log::debug;
use serde_json::Value;

use super::controllers::server::load_servers_lists;

pub mod config;
pub mod create;
pub mod delete;
pub mod init;
pub mod start;

#[doc = "返回输入的内容"]
pub fn input() -> String {
    io::stdout().flush().expect("无法刷新stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取 stdin 失败");
    input = input.trim().to_string();
    input
}

#[doc = "返回用户选择的服务器"]
pub fn choose_server(description: &str) -> Value {
    let server_configs = load_servers_lists();

    loop {
        let mut index = 0;
        let mut server_names = Vec::<&String>::new();
        let server_configs_clone = server_configs.clone();
        if let Some(server) = server_configs_clone.as_object() {
            for (server, _) in server {
                println!("{index}: {server}");
                server_names.push(server);
                index += 1;
            }
        }
        if index == 0 {
            return Value::Null;
        }
        print!("请选择一个{description}的服务器: ");
        let input_value = input();
        match input_value.parse::<usize>() {
            Ok(value) => {
                if value > index {
                    println!("输入错误,请重新输入!");
                    continue;
                }
                let name = server_names[value];
                debug!("{server_configs}");
                return server_configs[name].clone();
            }
            Err(_) => {
                println!("输入错误,请重新输入!");
                continue;
            }
        }
    }
}

#[doc = "清空控制台"]
pub fn clear_console() {
    let term = Term::stdout();
    term.clear_screen().expect("Failed to clear screen");
}
