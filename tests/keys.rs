/*
 * Copyright (c) 2024 MCSCS-Rust.
 */

use mcscs::pages::{clear_console, pause};

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
