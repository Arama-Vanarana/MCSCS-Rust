/*
 * Copyright (c) 2024 Minecraft Server Config Script for Rust.
 */

use mcscs::{java::detect_java, pages::init};

/// 测试寻找Java环境
#[tokio::test]
async fn test_detect_java() {
    init::main().await.expect("main()");
    println!("{}", serde_json::to_string_pretty(&detect_java()).unwrap())
}
