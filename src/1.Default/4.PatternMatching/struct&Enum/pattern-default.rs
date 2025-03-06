fn main() {
    let value = 3;

    match value {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Not one and two or three"), // 기본값
    }
}