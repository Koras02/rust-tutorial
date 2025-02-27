fn main() {
    let s = String::from("Hello");

    let r1 = &s; // 불변 참조
    let r2 = &s; // 또 다른 불변 참조

    println!("r1: {}, r2 {}", r1,r2);
}
