fn print_length(s: &str) {
    println!("Length: {}", s.len());
}

fn main() {
    let my_string = String::from("Hello");
    print_length(&my_string); 
}
