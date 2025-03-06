#[allow(dead_code)]
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
}

fn main() {
    let msg = Message::ChangeColor(255,0,0);

    match msg {
        Message::Quit => println!("Quit!"),
        Message::ChangeColor(r, g, b) => {
            println!("Change Color to Red: {}, green: {}, blue: {}", r,g,b);
        }
        Message::Move { x, y } => {
            println!("Move to x: {}, y: {}", x, y);
        }
    }
}