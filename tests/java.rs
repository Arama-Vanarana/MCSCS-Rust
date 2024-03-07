use mcscs::{java::detect_java, pages::init};

/// 测试寻找Java环境
#[test]
fn test_detect_java() {
    println!("{}", serde_json::to_string_pretty(&detect_java()).unwrap())
}

/// 测试寻找Java环境(带日志)
#[tokio::test]
async fn test_log_detect_java() {
    if let Err(err) = init::main().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    println!("{}", serde_json::to_string_pretty(&detect_java()).unwrap())
}
