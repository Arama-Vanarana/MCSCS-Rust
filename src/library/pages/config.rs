use serde_json::json;

use crate::library::controllers::input;
use crate::library::controllers::server::save_servers_lists;
use crate::library::pages::choose_server;
use crate::library::pages::create::{encoding, jvm_args, xms, xmx};

pub fn main() {
    let mut server = choose_server("需要配置");
    println!("{}", server);
    let server_name = &server["name"].as_str().unwrap().to_string();
    loop {
        println!("1: Xms: JVM初始堆内存");
        println!("2: Xmx: JVM最大堆内存");
        println!("3: encoding: 输入和输出的编码");
        println!("4: JVM args: 其他JVM参数");
        println!("0: 返回");
        print!("请选择一个选项: ");
        let input_value = input();
        if input_value == "1" {
            server["Xms"] = json!(xms(Some(server["Xmx"].as_u64().unwrap())));
            save_servers_lists(server_name, Some(&server));
        } else if input_value == "2" {
            server["Xmx"] = json!(xmx(server["Xms"].as_u64().unwrap()));
            save_servers_lists(server_name, Some(&server));
        } else if input_value == "3" {
            server["encoding"] = json!(encoding());
            save_servers_lists(server_name, Some(&server));
        } else if input_value == "4" {
            server["jvm_args"] = json!(jvm_args(Some(&server["jvm_args"])));
            save_servers_lists(server_name, Some(&server));
        } else if input_value == "0" {
            break;
        } else {
            println!("输入错误,请重新输入!");
        }
    }
}