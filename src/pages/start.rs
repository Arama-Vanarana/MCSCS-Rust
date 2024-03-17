/*
 * Copyright (c) 2024 Arama. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */

use std::error::Error;
use std::{env, fs, path::Path, process::Command};

use dialoguer::Confirm;
use log::trace;

use crate::select::select_server;

/// 如果path路径参数内没有找到eula.txt(不会寻找子文件夹), 就会要求用户同意EULA协议(https://aka.ms/MinecraftEULA)
pub fn eula(path: &Path) -> Result<(), Box<dyn Error>> {
    if fs::metadata(path.join("eula.txt")).is_err() {
        if Confirm::new()
            .with_prompt("你是否同意Minecraft EULA(https://aka.ms/MinecraftEULA)?")
            .interact()
            .unwrap()
        {
            let time = chrono::Local::now().format("%a %b %d %H:%M:%S %Z %Y");
            let contents = format!("# Create By Minecraft Server Config Script\n# By changing the setting below to TRUE you are indicating your agreement to Minecraft EULA(https://aka.ms/MinecraftEULA).\n# {time}\neula=true");
            fs::write(path.join("eula.txt"), contents)?
        } else {
            return Err("必须同意EULA".into());
        }
    }
    Ok(())
}

/// 启动服务器页面
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut server = select_server();
    if server.is_null() {
        println!("你还没有创建任何一个服务器!");
        return Ok(());
    }
    let name = server["name"].as_str().unwrap();
    let current_dir = env::current_dir()?.join("MCSCS").join("servers").join(name);
    eula(&current_dir)?;
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
    trace!("shell <- {}", format!("{:?}", process));

    process.spawn().expect("main()").wait().expect("main()");

    Ok(())
}
