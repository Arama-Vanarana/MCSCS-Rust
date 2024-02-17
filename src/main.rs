pub(crate) mod library;
fn main() {
    if !library::pages::init::main().is_ok() {
        return;
    }
    let start_time = std::time::Instant::now();

    println!(
        "{}",
        serde_json::to_string_pretty(&library::controllers::java::detect_java()).unwrap()
    );

    let elapsed_time = start_time.elapsed();
    let elapsed_ms = elapsed_time.as_millis();
    let elapsed_secs = elapsed_time.as_secs();

    println!("运行时间: {}毫秒 {}秒", elapsed_ms, elapsed_secs);
}
