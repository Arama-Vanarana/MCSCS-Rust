/*
 * Copyright (c) 2024 Arama. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */

use mcscs::{java::detect_java, pages::init};

/// 测试寻找Java环境
#[tokio::test]
async fn test_detect_java() {
    init::main().await.expect("main()");
    println!("{}", serde_json::to_string_pretty(&detect_java()).unwrap())
}
