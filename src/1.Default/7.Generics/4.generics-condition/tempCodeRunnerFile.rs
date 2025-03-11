fn print_length<T: std::ops::Deref<Target = str>>(s: T) {
    println!("Length: {}", s.len());
}

fn main() {
    let Hello = String::from("Hello!");
    print_length(Hello);
}