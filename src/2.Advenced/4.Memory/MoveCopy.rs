
fn main() {
    let s1 = String::from("Hello"); 
    let s2 = s1; 

// println!("{}", s1); // 🚫 에러: s1은 더 이상 유효하지 X

    let x = 5;
    let y = x; // x의 값이 복사 
    println!("{}", x); // ✅ 에러 X
}