mod library;

#[tokio::main]
async fn main() {
    if !library::pages::init::main().await.is_ok() {
        return;
    }
    let start_time = std::time::Instant::now();
    library::controllers::java::save_java_lists(library::controllers::java::detect_java());
    println!(
        "{}",
        serde_json::to_string_pretty(&library::controllers::java::load_java_lists()).unwrap()
    );

    let elapsed_time = start_time.elapsed();
    println!(
        "运行时间: {}毫秒 {}秒",
        elapsed_time.as_millis(),
        elapsed_time.as_secs()
    );
}
