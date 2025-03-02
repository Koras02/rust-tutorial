fn main() {
    let mut s = String::from("Hello");

    change(&mut s); // 가변 참조

    println!("{}", s);

}


fn change(s: &mut String) {
    s.push_str(", World");
}