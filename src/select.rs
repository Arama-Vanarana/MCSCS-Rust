/*
 * Copyright (c) 2024 Arama. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use log::info;
use serde_json::Value;

use crate::server::load_servers_lists;

/// 让用户选择一个选项
///
/// # 使用
/// ```
/// use mcscs::select::select_option;
/// let options = vec!["选项1", "选项2", "选项3"];
/// let selection = select_option("请选择一个选项", &options);
/// todo!("处理选择的选项");
/// ```
pub fn select_option<T: ToString>(description: &str, items: &[T]) -> Result<usize, Box<dyn Error>> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(description)
        .items(items)
        .interact()?;
    info!("user -> {selection}");
    Ok(selection)
}

/// 让用户选择一个文件
///
/// # 使用
/// ```
/// use mcscs::select::select_file;
/// let file_path = select_file("请选择任意一个文件");
/// todo!("处理选择的文件");
/// ```
pub fn select_file(description: &str) -> Result<PathBuf, Box<dyn Error>> {
    let mut current_dir = fs::canonicalize(".")?;

    loop {
        let entries: Vec<(PathBuf, bool)> = fs::read_dir(&current_dir)?
            .filter_map(|entry| entry.ok().map(|e| (e.path(), e.path().is_dir())))
            .collect();

        let mut options = entries
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
            .collect::<Vec<String>>();

        options.insert(0, String::from(".. (dir)"));

        let selection = select_option(
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
pub fn select_server() -> Value {
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
    let selection = select_option("请选择一个服务器", &server_names).expect("");
    let name = server_names[selection];
    server_configs[name].clone()
}
