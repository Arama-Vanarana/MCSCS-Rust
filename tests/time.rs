/*
 * Copyright (c) 2024 Arama.
 */

#[test]
fn test_get_now_time() {
    let time = chrono::Local::now();
    println!("{time}")
}
