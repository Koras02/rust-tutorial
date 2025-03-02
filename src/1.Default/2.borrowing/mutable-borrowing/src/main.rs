fn main() {
    let mut s = String::from("Hello");

    let r1 = &mut s; // 가변 참조
    r1.push_str(", World!"); // r1을 통해 s를 수정

    println!("{}", r1); // "Hello, World!"
}
