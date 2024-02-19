pub mod init; // 初始化

pub fn input() -> String { // rust版的cin(?)
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("read_line error!");
    input
}
