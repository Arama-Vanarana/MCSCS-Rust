use std::{env, fs, io::Read};

use log::debug;
use serde_json::{json, Value};

/// 保存服务器配置到`MCSCS\configs\servers.json`
///
/// # 使用
/// * 修改(如果服务器不存在就会创建)
/// ```
/// use serde_json::Value;
/// use mcscs::server::{load_servers_lists, save_servers_lists};
/// let mut server = load_servers_lists()["name"].take(); // 获取已经保存的配置
/// server["Xms"] = Value::from(100000000);
/// save_servers_lists("name", Some(&server));
/// ```
/// * 删除
/// ```
/// use mcscs::server::save_servers_lists;
/// save_servers_lists("name", None);
/// ```
pub fn save_servers_lists(server: &str, config: Option<&Value>) {
    let mut data = load_servers_lists();
    let current_dir = env::current_dir().unwrap().join("MCSCS").join("configs");
    let file = fs::File::create(current_dir.clone().join("servers.json"))
        .expect("创建MCSCS/configs/servers.json失败");
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
    serde_json::to_writer_pretty(file, &json!(data)).expect("写入MCSCS/configs/servers.json错误");
}

/// 从[`MCSCS\configs\servers.json`读取所有服务器配置
pub fn load_servers_lists() -> Value {
    let mut file = fs::File::open(
        env::current_dir()
            .unwrap()
            .join("MCSCS")
            .join("configs")
            .join("servers.json"),
    )
    .expect("读取MCSCS/configs/servers.json失败");

    // 读取文件内容到字符串中
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("读取MCSCS/configs/servers.json失败");
    let data = serde_json::from_str::<Value>(&json_data).expect("无法解析 JSON");
    debug!("从MCSCS/configs/servers.json加载到的服务器配置: {data}");
    data
}
