use reqwest::Url;

fn main() {
    let url = Url::parse("https://download.fastmirror.net/api/v3").expect("get_fastmirror_value()");
    println!("{url}");
}
