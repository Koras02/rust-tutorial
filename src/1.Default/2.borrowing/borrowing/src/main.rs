fn main() {
    let mut x = 5;

    // 불변 빌림
    let y = &x; // x를 불변으로 빌림
    println!("y: {}", y); // y는 x의 값을 읽을 수 있음

    // 가변 빌림
    let z = &mut x; // x를 가변으로 빌림
    *z += 1; // z를 통해 x의 값을 변경할 수 있음
    println!("x: {}", x); // x의 값은 6
}
