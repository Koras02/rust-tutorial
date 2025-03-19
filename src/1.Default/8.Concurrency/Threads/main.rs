use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Threads: {}", i);
        }
    });

    for i in 1..3 {
        println!("Main Threads: {}", i);
    }

    handle.join().unwrap(); // Treads end..wait
}