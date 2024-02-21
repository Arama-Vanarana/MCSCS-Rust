mod library;

#[tokio::main]
async fn main() {
    if !library::pages::init::main().await.is_ok() {
        return;
    }
}

#[cfg(test)]
mod test;