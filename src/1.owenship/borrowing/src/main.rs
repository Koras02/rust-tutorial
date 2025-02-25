fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1); // 불변 참조

    println!("Length: {}", len); // s1 여전히 유효
}

fn calculate_length(s: &String) -> usize {
    s.len() 
}
