use std::{env, fmt::format, path::PathBuf, process::Command};

use serde_json::{json, Value};

use crate::library::controllers::{
    input,
    server::{self, load_servers_lists},
};

pub fn main() {
    let mut server_configs = load_servers_lists();
    loop {
        let mut index = 0;
        let mut server_names = Vec::<&String>::new();
        let server_configs_clone = server_configs.clone();
        if let Some(server) = server_configs_clone.as_object() {
            for (server, _) in server {
                println!("{index}: {server}");
                server_names.push(server);
                index += 1;
            }
        }
        print!("请选择一个需要启动的服务器: ");
        let input_value = input();
        match input_value.parse::<usize>() {
            Ok(value) => {
                if value > index {
                    println!("输入错误,请重新输入!");
                    continue;
                }
                let name = server_names[value];
                let mut server = server_configs[name].take();
                let mut jvm_args = server["jvm_args"].take();
                let args = jvm_args.as_array_mut().unwrap();
                args.push(json!(format!("-Xms{}", server["Xms"].take())));
                args.push(json!(format!("-Xmx{}", server["Xmx"].take())));
                args.push(json!(format!(
                    "-Dfile.encoding{}",
                    server["encoding"].take().as_str().unwrap()
                )));
                args.push(json!("-jar"));
                args.push(json!("server.jar"));
                args.push(json!("nogui"));
                let mut string_args = Vec::<&str>::new();
                string_args.push("/C");
                string_args.push("start");
                string_args.push(server["java"]["path"].as_str().unwrap());
                for arg in args {
                    string_args.push(arg.as_str().unwrap());
                }
                Command::new("cmd.exe")
                    .args(&string_args)
                    .current_dir(
                        env::current_dir()
                            .unwrap()
                            .join("MCSCS")
                            .join("servers")
                            .join(name),
                    )
                    .spawn()
                    .unwrap();
                println!("{}", json!(string_args));
                break;
            }
            Err(_) => {
                println!("输入错误,请重新输入!");
                continue;
            }
        }
    }
}
