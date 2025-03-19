use async_std::task;

fn main() {
    task::block_on(async {
        let task1 = task::spawn(async {
            for i in 1..5 {
                println!("Async Work 1: {}", i);
                async_std::task::sleep(std::time::Duration::from_millis(500)).await; // 0.5 sec wait 
            }
        });

        let task2 = task::spawn(async {
            for i in 1..3 {
                println!("Async Work 2: {}", i);
                async_std::task::sleep(std::time::Duration::from_millis(300)).await; // 0.3 set wait 
            }
        });

        task1.await;
        task2.await;
    })
}