use crate::library::controllers::java::detect_java;

#[tokio::test]
#[doc = "测试寻找Java环境"]
async fn test_detect_java() {
    println!(
        "{}",
        serde_json::to_string_pretty(&detect_java()).unwrap()
    )
}