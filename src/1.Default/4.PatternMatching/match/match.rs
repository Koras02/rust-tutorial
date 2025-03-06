fn main() {
    let number = 2;
    
    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        4 => println!("Four!"),
        5 => println!("Five!"),
        6..=10 => println!("Between Seven and ten"),
        _ => println!("Something else"), // Default Pattern
    }
}