/*
 * Copyright (c) 2024 Minecraft Server Config Script for Rust.
 */

use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use log::{error, info};
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
/// let input_value = input("请输入任意内容");
/// ```
pub fn input(description: &str) -> String {
    dialoguer::Input::<String>::new()
        .with_prompt(description)
        .interact_text()
        .unwrap()
}

pub fn choose<T: ToString>(description: &str, items: &[T]) -> Result<usize, Box<dyn Error>> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(description)
        .items(items)
        .interact()?;
    info!("user -> {selection}");
    Ok(selection)
}

pub fn choose_file(description: &str) -> Result<PathBuf, Box<dyn Error>> {
    let mut current_dir = fs::canonicalize(".")?;

    loop {
        let entries: Vec<(PathBuf, bool)> = fs::read_dir(&current_dir)?
            .filter_map(|entry| entry.ok().map(|e| (e.path(), e.path().is_dir())))
            .collect();

        let mut options: Vec<String> = entries
            .iter()
            .map(|(path, is_dir)| {
                let mut display = String::from(
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or(""),
                );
                if *is_dir {
                    display.push_str(" (dir)");
                }
                display
            })
            .collect();

        options.insert(0, String::from(".. (dir)"));

        let selection = choose(
            &format!("{} {description}", current_dir.display()),
            &options,
        )
        .unwrap();

        if selection == 0 {
            // 用户选择了返回上一级目录的选项
            current_dir.pop();
        } else {
            let (selected_entry, is_dir) = entries.get(selection - 1).unwrap(); // 减去“..”选项的索引
            if *is_dir {
                // 如果用户选择的是文件夹，则进入该文件夹
                current_dir = selected_entry.clone();
            } else {
                return Ok(selected_entry.clone());
            }
        }
    }
}

/// 返回用户选择的服务器配置
pub fn choose_server() -> Value {
    let server_configs = load_servers_lists(None);
    let mut server_names = Vec::<&String>::new();
    if let Some(server) = server_configs.as_object() {
        for (server, _) in server {
            server_names.push(server);
        }
    }
    if server_names.is_empty() {
        return Value::Null;
    }
    let selection = choose("请选择一个服务器", &server_names).expect("");
    let name = server_names[selection];
    server_configs[name].clone()
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
