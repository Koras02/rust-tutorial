fn duplicate<T: Clone>(value: T) -> (T, T) {
    (value.clone(), value)
}

fn main() {
    let number = 10;
    let (number1, number2) = duplicate(number);
    println!("Duplicate number: ({} and {})", number1, number2);

    let text = String::from("Hello");
    let (text1, text2) = duplicate(text);
    println!("Duplicate text: ({} and {})", text1, text2);
}