pub(crate) mod library;

// #[tokio::main]
// async
fn main() {
    // if !library::pages::init::main().await.is_ok() {
    //     return;
    // }
    let start_time = std::time::Instant::now();

    let java = library::controllers::java::detect_java();
    println!("{}", serde_json::to_string_pretty(&java).unwrap());

    let elapsed_time = start_time.elapsed();
    println!(
        "运行时间: {}毫秒 {}秒",
        elapsed_time.as_millis(),
        elapsed_time.as_secs()
    );
}
