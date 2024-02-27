use console::Term;
use log::debug;
use serde_json::Value;

use crate::library::controllers::{input, server::load_servers_lists};

pub mod config;
// 配置服务器
pub mod create;
// 创建服务器
pub mod init;
// 初始化
pub mod start; //启动服务器

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

pub fn clear_console() {
    let term = Term::stdout();
    term.clear_screen().expect("Failed to clear screen");
}