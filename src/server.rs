use std::{env, fs, io::Read};

use log::trace;
use serde_json::{json, Value};

/// 保存服务器配置到`MCSCS\configs\servers.json`
///
/// # 使用
/// * 修改(如果服务器不存在就会创建)
/// ```
/// use serde_json::json;
/// use mcscs::pages::create::to_bytes;
/// use mcscs::server::{load_servers_lists, save_servers_lists};
/// let mut server = load_servers_lists(None)["server_name"].take(); // 获取已经保存的配置
/// server["Xms"] = json!(to_bytes("1G"));
/// save_servers_lists("name", &server);
/// ```
pub fn save_servers_lists(server: &str, config: &Value) {
    let path = env::current_dir()
        .expect("save_servers_lists()")
        .join("MCSCS")
        .join("servers")
        .join(server)
        .join("config.json");
    let file = fs::File::create(&path).expect("save_servers_lists()");
    trace!("{} <- {config}", path.display());
    serde_json::to_writer_pretty(file, &config).expect("save_servers_lists()");
}

/// 从[`MCSCS\configs\servers.json`]读取所有服务器配置
pub fn load_servers_lists(server: Option<&str>) -> Value {
    let path = env::current_dir()
        .expect("load_servers_lists()")
        .join("MCSCS")
        .join("servers");
    // let mut file = fs::File::open(
    //     env::current_dir()
    //         .unwrap()
    //         .join("MCSCS")
    //         .join("servers")
    //         .join(server)
    //         .join("config.json"),
    // )
    // .expect("load_servers_lists()");

    // 读取文件内容到字符串中
    // let mut json_data = String::new();
    // file.read_to_string(&mut json_data)
    //     .expect("load_servers_lists()");
    // let data = serde_json::from_str::<Value>(&json_data).expect("load_servers_lists()");
    // trace!("MCSCS/configs/servers.json -> {data}");
    // data
    if let Some(server) = server {
        let path = path.join(server).join("config.json");
        let mut data = String::new();
        if fs::File::open(&path).is_err() {
            fs::File::create(&path).expect("load_servers_lists()");
        }
        let mut file = fs::File::open(&path).expect("load_servers_lists()");
        file.read_to_string(&mut data)
            .expect("load_servers_lists()");

        let data = serde_json::from_str::<Value>(&data).expect("load_servers_lists()");
        trace!("{} -> {data}", path.display());
        data
    } else {
        let mut configs = json!({});
        let entries = fs::read_dir(&path).expect("load_servers_lists()");
        for entry in entries.flatten() {
            let path = entry.path().join("config.json");
            if entry.path().is_dir() {
                let mut data = String::new();
                if fs::File::open(&path).is_err() {
                    fs::File::create(&path).expect("load_servers_lists()");
                }
                let mut file = fs::File::open(&path).expect("load_servers_lists()");
                file.read_to_string(&mut data)
                    .expect("load_servers_lists()");

                let data = serde_json::from_str::<Value>(&data).expect("load_servers_lists()");
                trace!("{} -> {data}", path.display());
                configs[data["name"].as_str().expect("load_servers_lists()")] = data.clone();
            }
        }
        configs
    }
}
