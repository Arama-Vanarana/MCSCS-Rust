use crate::library::{
    controllers::input,
    pages::{create, init, start},
};

mod library;

#[tokio::main]
async fn main() {
    if let Err(err) = init::main().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    loop {
        println!("1: 启动服务器");
        println!("2: 创建服务器");
        println!("0: 退出");
        print!("请选择一个选项: ");
        let input_value = input();
        if input_value == "1" {
            start::main();
        } else if input_value == "2" {
            create::main().await;
        } else if input_value == "0" {
            return;
        }
    }
}

#[cfg(test)]
mod test;
