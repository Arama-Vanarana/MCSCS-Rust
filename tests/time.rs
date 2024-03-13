/*
 * Copyright (c) 2024 Minecraft Server Config Script for Rust.
 */

#[test]
fn test_get_now_time() {
    let time = chrono::Local::now();
    println!("{time}")
}
