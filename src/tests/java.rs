use crate::library::{controllers::java::detect_java, pages::init};

#[tokio::test]
#[doc = "测试寻找Java环境"]
async fn test_detect_java() {
    if let Err(err) = init::main().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&detect_java()).unwrap()
    );
    ()
}