/*
 * Copyright (c) 2024 MCSCS-Rust.
 */

#[test]
fn test_get_now_time() {
    let time = chrono::Local::now();
    println!("{time}")
}
