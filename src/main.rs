use log::info;

mod library;

#[tokio::main]
async fn main() {
    if !library::pages::init::main().await.is_ok() {
        return;
    }
    let start_time = std::time::Instant::now();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("read_line error!");
    println!("你输入的内容是 : {}", input);
    let elapsed_time = start_time.elapsed();
    println!(
        "运行时间: {}毫秒 {}秒",
        elapsed_time.as_millis(),
        elapsed_time.as_secs()
    );
}

// use log::{error, info};
// use log4rs;

// fn main() {
//     // 从log4rs配置文件中加载配置
//     log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

//     // 记录一些日志
//     info!("This is an information message");
//     error!("This is an error message");
// }
