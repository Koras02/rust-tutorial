fn main() {
    let mut s = String::from("Hello");

    let r1 = &s; // 불변 참조
    let r2 = &s; // 또 다른 불변 참조
    println!("r1: {}, r2: {}", r1, r2); // ✅ 에러 X
    // println!("r3: {}", r3); // 🚫 에러: r3는 아직 유효하지 X

    let r3 = &mut s; // 가변 참조 (이 시점부터 불변 참조 X)
    r3.push_str(", World!"); // s를 변경
    println!("r3: {}", r3); // ✅ 에러 X)
}