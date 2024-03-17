/*
 * Copyright (c) 2024 Arama. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */
use std::io;
use std::io::Write;

use console::Term;
use dialoguer::theme::ColorfulTheme;
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
    dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
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
