use jsonrpc::Client;
use mcscs::{aria2c::install_aria2c, pages::init};
use serde_json::json;

#[tokio::test]
async fn test_get_aria2c_version() {
    init::main().await.expect("main()");
    let client = Client::simple_http("http://127.0.0.1:6800/jsonrpc", None, None)
        .expect("test_get_aria2c_version()");
    let args = jsonrpc::arg(json!(["token:MCSCS"]));
    let request = client.build_request("aria2.getVersion", Some(&args));
    match Client::send_request(&client, request) {
        Ok(result) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&json!(result.result)).unwrap()
            );
        }
        Err(e) => {
            println!("{e}");
        }
    }
}

#[tokio::test]
async fn test_install_aria2c() {
    install_aria2c().await;
}
