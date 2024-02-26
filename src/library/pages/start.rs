use crate::library::controllers::{input, server::load_servers_lists};
use serde_json::json;
use std::{env, os::windows::process::CommandExt, process::Command};

pub fn main() {
    let mut server_configs = load_servers_lists();
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
        print!("请选择一个需要启动的服务器: ");
        let input_value = input();
        match input_value.parse::<usize>() {
            Ok(value) => {
                if value > index {
                    println!("输入错误,请重新输入!");
                    continue;
                }
                let name = server_names[value];
                let mut server = server_configs[name].take();

                let mut process = Command::new("cmd.exe");
                // 执行目录
                process.current_dir(
                    env::current_dir()
                        .unwrap()
                        .join("MCSCS")
                        .join("servers")
                        .join(name),
                );
                process.arg("/C"); // 服务器关闭后自动退出
                process.arg("start"); // 启动新窗口
                process.raw_arg(format!("\"{name}\"")); // 标题
                process.arg(server["java"]["path"].take().as_str().unwrap()); // java.exe
                                                                              // 在配置文件设置的JVM参数
                for arg in server["jvm_args"].take().as_array_mut().unwrap() {
                    process.arg(arg.as_str().unwrap());
                }
                process.arg(format!("-Xms{}", server["Xms"].take())); // JVM初始堆内存
                process.arg(format!("-Xmx{}", server["Xmx"].take())); // JVM最大堆内存
                process.arg(format!( // 输出和输入的编码格式
                    "-Dfile.encoding={}",
                    server["encoding"].take().as_str().unwrap()
                ));
                process.arg("-jar"); // 使用Jar
                process.arg("server.jar"); // Jar路径
                process.arg("nogui"); // 禁用GUI
                process.spawn().expect("服务器启动失败");
                break;
            }
            Err(_) => {
                println!("输入错误,请重新输入!");
                continue;
            }
        }
    }
}
