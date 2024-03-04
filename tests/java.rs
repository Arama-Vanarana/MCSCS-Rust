use mcscs::java::detect_java;

#[test]
#[doc = "测试寻找Java环境"]
fn test_detect_java() {
    println!("{}", serde_json::to_string_pretty(&detect_java()).unwrap())
}
