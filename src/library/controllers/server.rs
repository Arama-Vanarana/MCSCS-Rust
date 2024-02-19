use serde_json::{json, Value};

pub fn get_all_server_configs() -> Value {
    use std::io::Read;
    let mut file = std::fs::File::open(
        std::env::current_dir()
            .unwrap()
            .join("servers")
            .join("config.json"),
    )
    .expect("读取config.json失败");

    // 读取文件内容到字符串中
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("读取config.json失败");

    // 将 JSON 字符串反序列化为 MyData 结构体实例
    let data: serde_json::Value = serde_json::from_str(&json_data).expect("无法解析 JSON");
    data["data"].clone()
}

// ! 如果config参数是None就会删除配置
pub fn write_server_config(server: &str, config: Option<Value>) -> Value {
    let mut all_config = get_all_server_configs();
    let current_dir = std::env::current_dir().unwrap().join("servers");
    let file = std::fs::File::create(current_dir.clone().join("config.json"))
        .expect("创建config.json失败");
    match config {
        Some(c) => {
            all_config[server] = json!(c);
        }
        None => {
            match &mut all_config {
                Value::Object(ref mut map) => {
                    map.remove(server);
                }
                _ => {} // 如果 JSON 不是对象类型，则不需要删除键
            }
        }
    };
    serde_json::to_writer_pretty(file, &json!({"data": all_config})).expect("写入config.json错误");
    json!(all_config)
}
