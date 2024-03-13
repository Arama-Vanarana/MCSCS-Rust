/*
 * Copyright (c) 2024 Minecraft Server Config Script for Rust.
 */

use std::{env, fs, path::Path, process::Command};

use crate::pages::{choose_server, input};

/// 如果path路径参数内没有找到eula.txt(不会寻找子文件夹), 就会要求用户同意EULA协议(https://aka.ms/MinecraftEULA)
pub fn eula(path: &Path) {
    if fs::metadata(path.join("eula.txt")).is_err() {
        loop {
            print!("你是否同意Minecraft EULA(https://aka.ms/MinecraftEULA)?(Y/N) ");
            let input_value = input().to_lowercase();
            if input_value == "y" || input_value == "yes" {
                let time = chrono::Local::now().format("%a %b %d %H:%M:%S %Z %Y");
                let contents = format!("# Create By Minecraft Server Config Script\n# By changing the setting below to TRUE you are indicating your agreement to Minecraft EULA(https://aka.ms/MinecraftEULA).\n# {time}\neula=true");
                match fs::write(path.join("eula.txt"), contents) {
                    Ok(_) => break,
                    Err(e) => {
                        println!("写入eula.txt失败: {}", e);
                        return;
                    }
                }
            }
            if input_value == "n" || input_value == "no" {
                return;
            }
            println!("输入错误,请重新输入!");
        }
    }
}

/// 启动服务器页面
pub fn main() {
    let mut server = choose_server("需要启动");
    if server.is_null() {
        println!("你还没有创建任何一个服务器!");
        return;
    }
    let name = server["name"].as_str().unwrap();
    let current_dir = env::current_dir()
        .unwrap()
        .join("MCSCS")
        .join("servers")
        .join(name);
    eula(&current_dir);
    let mut process = Command::new(server["java"]["path"].as_str().unwrap());
    process.current_dir(current_dir);
    for arg in server["jvm_args"].as_array_mut().unwrap() {
        // 在配置文件设置的JVM参数
        process.arg(arg.as_str().unwrap());
    }
    process.arg(format!("-Xms{}", server["Xms"])); // JVM初始堆内存
    process.arg(format!("-Xmx{}", server["Xmx"])); // JVM最大堆内存
    process.arg(format!(
        // 输出和输入的编码格式
        "-Dfile.encoding={}",
        server["encoding"].as_str().unwrap()
    ));
    process.arg("-jar"); // 使用Jar
    process.arg("server.jar"); // Jar路径
    for arg in server["server_args"].as_array_mut().unwrap() {
        // 在配置文件设置的服务器参数
        process.arg(arg.as_str().unwrap());
    }
    process.spawn().expect("服务器启动失败");
}
