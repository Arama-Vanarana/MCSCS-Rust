use std::io::{self, Write};

use log::{debug, error};
use serde_json::Value;

use crate::server::load_servers_lists;

pub mod config;
pub mod create;
pub mod delete;
pub mod import;
pub mod init;
pub mod start;

/// 返回输入的内容
///
/// # 使用
/// ```
/// use mcscs::pages::input;
/// print!("请输入任意内容: ");
/// let input_value = input();
/// ```
pub fn input() -> String {
    io::stdout().flush().expect("无法刷新stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取 stdin 失败");
    input = input.trim().to_string();
    input
}

/// 返回用户选择的服务器配置
pub fn choose_server(description: &str) -> Value {
    let server_configs = load_servers_lists(None);

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
                if value >= index {
                    println!("输入错误,请重新输入!");
                    continue;
                }
                let name = server_names[value];
                debug!("{server_configs}");
                return server_configs[name].clone();
            }
            Err(e) => {
                error!("{e}");
                println!("输入错误,请重新输入!");
                continue;
            }
        }
    }
}

/// 清空控制台, 类似在cmd执行cls命令
pub fn clear_console() {
    if let Err(e) = console::Term::stdout().clear_screen() {
        error!("{e}");
    }
}

/// 暂停程序, 类似在cmd执行pause命令
pub fn pause() {
    print!("请按任意键继续...");
    let _ = io::stdout().flush();
    if let Err(e) = console::Term::stdout().read_key() {
        error!("{e}");
    }
}
