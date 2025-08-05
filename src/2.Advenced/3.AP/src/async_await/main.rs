// main.rs

#[tokio::main]
async fn main() {
    say_hello().await;
}

async fn say_hello() {
    println!("안녕, 비s동기 세상!");
}
