use crate::library::{
    controllers::server::save_servers_lists,
    pages::{choose_server, input},
};

pub fn main() {
    let server = choose_server("需要删除");
    if server.is_null() {
        println!("你还没有创建任何一个服务器!");
        return;
    }
    loop {
        print!("确认删除?(Y/N) ");
        let input_value = input().to_lowercase();
        if input_value == "y" || input_value == "yes" {
            save_servers_lists(server["name"].as_str().unwrap(), None);
            break;
        } else if input_value == "n" || input_value == "no" {
            break;
        } else {
            println!("输入错误,请重新输入!");
        }
    }
}
