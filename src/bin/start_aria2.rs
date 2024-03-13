/*
 * Copyright (c) 2024 MCSCS-Rust.
 */

use std::{env, fs, process::Command};

use chrono::Local;

fn main() {
    let current_dir = env::current_dir().unwrap().join("MCSCS");
    let log_path = current_dir
        .join("logs")
        .join(Local::now().format("%Y%m%d%H%M").to_string());
    fs::create_dir_all(&log_path).expect("创建logs文件夹失败");
    fs::create_dir_all(current_dir.join("downloads")).expect("创建MCSCS/downloads文件夹失败");
    #[cfg(target_os = "windows")]
    let execute = current_dir.join("aria2c").join("aria2c.exe");
    #[cfg(not(any(target_os = "windows")))]
    let execute = "aria2c";
    let mut aria2c = Command::new(execute);
    aria2c.arg(format!("--dir={}", current_dir.join("downloads").display()));
    aria2c.arg(format!("--log={}", log_path.join("aria2c.log").display()));
    aria2c.arg("--enable-rpc=true");
    aria2c.arg("--rpc-listen-port=6800");
    aria2c.arg("--rpc-max-request-size=10M");
    aria2c.arg("--rpc-secret=MCSCS");
    aria2c.arg(format!(
        "--conf-path={}",
        current_dir.join("aria2c").join("aria2c.conf").display()
    ));
    aria2c.arg("--console-log-level=info");
    // aria2c.arg("--quiet=true");
    aria2c
        .spawn()
        .expect("启动aria2c失败")
        .wait()
        .expect("等待aria2c退出失败");
}
