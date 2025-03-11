fn print_value<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}

fn main() {
    print_value(10); // 정수
    print_value(10.0); // 실수
    print_value("Hello, World!"); // 문자열
    print_value(true); // 불리언
}