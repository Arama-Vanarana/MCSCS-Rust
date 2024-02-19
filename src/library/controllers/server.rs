use log::debug;
use serde_json::{json, Value};

pub fn load_servers_lists() -> Value {
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
    debug!("从servers/config.json加载到的服务器配置: {}", data);
    data["data"].clone()
}

// ! 如果config参数是None就会删除配置
pub fn save_servers_lists(server: &str, config: Option<Value>) {
    let mut data = load_servers_lists();
    let current_dir = std::env::current_dir().unwrap().join("servers");
    let file = std::fs::File::create(current_dir.clone().join("config.json"))
        .expect("创建config.json失败");
    match config {
        Some(c) => {
            debug!("服务器配置更改: {} => {}", data[server], c);
            data[server] = c;
        }
        None => {
            match &mut data {
                Value::Object(ref mut map) => {
                    debug!("删除服务器: {}", server);
                    map.remove(server);
                }
                _ => {} // 如果 JSON 不是对象类型，则不需要删除键
            }
        }
    };
    debug!("已保存到servers/java.json: {}", data);
    serde_json::to_writer_pretty(file, &json!({"data": data})).expect("写入config.json错误");
}
