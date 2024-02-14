mod library;
use library::aria2_controller::download;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match download("https://dldir1.qq.com/qqfile/qq/PCQQ9.7.17/QQ9.7.17.29225.exe").await {
        Ok(path) => println!("{:?}", path),
        Err(e) => println!("{:?}", e),
    }
    Ok(())
}
