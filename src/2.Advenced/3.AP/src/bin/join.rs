use tokio::join;

async fn task1() {
    println!("Task 1 is running...");
}

async fn task2() {
    println!("Task 2 is running...")
}

#[tokio::main]
async fn main() {
    let ((), ()) = join!(task1(), task2());
    println!("Both tasks completed!");
}