use tokio::time::{sleep, Duration};

async fn do_something() {
    println!("Doing something...");
    sleep(Duration::from_secs(2)).await;
    println!("2 seconds later, done!");
}

#[tokio::main]
async fn main() {
    do_something().await;
}