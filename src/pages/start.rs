use std::{env, os::windows::process::CommandExt, process::Command};

use super::choose_server;

pub fn main() {
    let mut server = choose_server("需要启动");
    if server.is_null() {
        println!("你还没有创建任何一个服务器!");
        return;
    }
    let mut process = Command::new("cmd.exe");
    let name = server["name"].as_str().unwrap();
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
    process.arg(server["java"]["path"].as_str().unwrap()); // java.exe
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
    process.arg("nogui"); // 禁用GUI
    process.spawn().expect("服务器启动失败");
}
