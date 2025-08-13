fn main() {
    // 변수 선언
    let name = "Rust";
    let age = 6;

    // 출력
    println!("Hello, {}! Rust is {} years old", name, age);

    // 조건문
    if age > 5 {
        println!("Rust is still young!")
    }

    // 반복문 
    for i in 1..4 {
        println!("Iteration {}", i)
    }

    // 함수 호출
    let sum = add(3,5);
    println!("3 + 5 = {}", sum);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}


