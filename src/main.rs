mod library;

#[tokio::main]
async fn main() {
    if !library::pages::init::main().await.is_ok() {
        return;
    }
    let start_time = std::time::Instant::now();


    let elapsed_time = start_time.elapsed();
    println!(
        "运行时间: {}毫秒 {}秒",
        elapsed_time.as_millis(),
        elapsed_time.as_secs()
    );
}
