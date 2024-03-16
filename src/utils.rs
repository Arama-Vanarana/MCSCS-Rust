/*
 * Copyright (c) 2024 Arama.
 */
use std::io;
use std::io::Write;

use console::Term;
use log::error;

/// 返回输入的内容
///
/// # 使用
/// ```
/// use mcscs::utils::input;
/// let input_value = input("请输入任意内容");
/// todo!("处理输入值");
/// ```
pub fn input(description: &str) -> String {
    dialoguer::Input::<String>::new()
        .with_prompt(description)
        .interact_text()
        .unwrap()
}

/// 清空控制台, 类似运行Windows系统上的cls/类Unix系统上的clear命令
pub fn clear_console() {
    if let Err(e) = Term::stdout().clear_screen() {
        error!("{e}");
    }
}

/// 暂停程序, 类似运行Windows系统上的pause命令
pub fn pause() {
    print!("请按任意键继续...");
    if let Err(e) = io::stdout().flush() {
        error!("{e}");
    }
    if let Err(e) = Term::stdout().read_key() {
        error!("{e}");
    }
}
