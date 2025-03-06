fn main() {
    let some_value = Some(4);

    if let Some(x) = some_value {
        println!("Value: {}", x);
    } else {
        println!("No value found");
    }
}