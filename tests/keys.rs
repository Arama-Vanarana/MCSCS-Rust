/*
 * Copyright (c) 2024 Arama. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */

use mcscs::utils::{clear_console, pause};

/// 测试暂停函数
#[test]
fn test_pause() {
    pause()
}

/// 测试清空控制台函数
#[test]
fn test_clear_console() {
    print!("Test strings.");
    pause();
    clear_console();
}
