use std::{env, fs, io::Read};

use log::debug;
use serde_json::{json, Value};

/// 保存或删除指定服务器的配置
///
/// # 使用
/// ```
/// // 修改
/// let mut server = load_servers_lists()["name"].take(); // 获取已经保存的配置
/// server["Xms"] = 100000000
/// save_servers_lists("name", Some(&server));
///
/// // 删除
/// save_servers_lists("name", None);
/// ```
pub fn save_servers_lists(server: &str, config: Option<&Value>) {
    let mut data = load_servers_lists();
    let current_dir = env::current_dir().unwrap().join("MCSCS").join("servers");
    let file = fs::File::create(current_dir.clone().join("config.json"))
        .expect("创建MCSCS/servers/config.json失败");
    let server_config = data[server].take();

    match config {
        Some(c) => {
            debug!("服务器配置更改: {} -> {c}", server_config);
            data[server] = c.clone();
        }
        None => {
            if let Value::Object(ref mut map) = &mut data {
                debug!("服务器配置更改: {} -> None", server_config);
                map.remove(server);
            }
        }
    };
    serde_json::to_writer_pretty(file, &json!({"data": data}))
        .expect("写入MCSCS/servers/config.json错误");
}

/// 获取已经保存的所有服务器配置
/// # 使用
/// ```
/// let server = load_servers_lists();
/// ```
pub fn load_servers_lists() -> Value {
    let mut file = fs::File::open(
        env::current_dir()
            .unwrap()
            .join("MCSCS")
            .join("servers")
            .join("config.json"),
    )
    .expect("读取MCSCS/servers/config.json失败");

    // 读取文件内容到字符串中
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("读取MCSCS/servers/config.json失败");
    let data = serde_json::from_str::<Value>(&json_data).expect("无法解析 JSON")["data"].take();
    debug!("从MCSCS/servers/config.json加载到的服务器配置: {data}");
    data
}
