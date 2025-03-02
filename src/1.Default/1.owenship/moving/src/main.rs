fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // s1의 소유권이 s2로 이동

    println!("{}", s2); // 성공
    // println!("{}", s1) // 실패: s1은 더 이상 사용 X
}
