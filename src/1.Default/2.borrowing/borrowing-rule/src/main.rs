fn main() {

    let mut s = String::from("Hello");

    let r1 = &mut s; // 가변 참조
    let r2 = &s; // 불변 참조 (여기서 오류 발생)

    println!("{}, {}", r1, r2); // error: 불변 참조와 가변 참조를 동시에 사용
}
