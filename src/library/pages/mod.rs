pub mod init; // 初始化

#[doc = "返回输入的内容"]
pub fn input(desc: &str) -> String {
    use std::io::Write; 
    print!("{desc}: ");
    std::io::stdout().flush().expect("无法刷新stdout");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("读取 stdin 失败");
    input
}
