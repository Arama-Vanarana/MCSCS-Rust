/*
 * Copyright (c) 2024 Arama.
 */

use std::error::Error;

use serde_json::json;

use crate::pages::create::{encoding, jvm_args, server_args, xms, xmx};
use crate::select::{select_option, select_server};
use crate::server::save_servers_lists;
use crate::utils::clear_console;

/// 配置服务器页面
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut server = select_server();
    if server.is_null() {
        println!("你还没有创建任何一个服务器!");
        return Ok(());
    }
    let server_name = &server["name"].as_str().unwrap_or_default().to_string();

    let options = vec![
        "XMS(JVM初始堆内存)",
        "XMX(JVM最大堆内存)",
        "输入和输出的编码",
        "其他JVM参数",
        "服务器参数",
        "返回",
    ];
    loop {
        let selection = select_option("请选择一个选项", &options)?;
        if selection == options.len() - 1 {
            return Ok(());
        }
        clear_console();
        if selection == 0 {
            println!("1GiB = 1024MB, 1GB = 1000MB");
            println!("1MiB = 1024KB, 1MB = 1000KB");
            println!("1KiB = 1024Bytes, 1KB = 1000Bytes");
            server["Xms"] = json!(xms(Some(server["Xmx"].as_u64().unwrap_or_default())));
            save_servers_lists(server_name, &server);
        } else if selection == 1 {
            println!("1GiB = 1024MB, 1GB = 1000MB");
            println!("1MiB = 1024KB, 1MB = 1000KB");
            println!("1KiB = 1024Bytes, 1KB = 1000Bytes");
            server["Xmx"] = json!(xmx(server["Xms"].as_u64().unwrap_or_default()));
            save_servers_lists(server_name, &server);
        } else if selection == 2 {
            server["encoding"] = json!(encoding());
            save_servers_lists(server_name, &server);
        } else if selection == 3 {
            server["jvm_args"] = json!(jvm_args(Some(&server["jvm_args"])));
            save_servers_lists(server_name, &server);
        } else if selection == 4 {
            server["server_args"] = json!(server_args(Some(&server["server_args"])));
            save_servers_lists(server_name, &server);
        }
        clear_console();
    }
}
