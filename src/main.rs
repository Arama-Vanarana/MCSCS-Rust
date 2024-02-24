mod library;
use library::{controllers::input, pages::init};

#[tokio::main]
async fn main() {
    if !init::main().await.is_ok() {
        return;
    }
    println!("1: 启动服务器");
    println!("2: 创建服务器");
    print!("请选择一个选项: ");
    let input_value = input();
}

#[cfg(test)]
mod test;
