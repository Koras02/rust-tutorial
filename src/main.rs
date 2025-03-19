use tokio;

#[tokio::main]
async fn main() {
    let task1 = async {
        for i in 1..5 {
            println!("비동기 작업 1: {}", i);
        }
    };

    let task2 = async {
        for i in 1..3 {
            println!("비동기 작업 2: {}", i);
        }
    };

    tokio::join!(task1, task2); // 두 작업을 동시에 실행
}
