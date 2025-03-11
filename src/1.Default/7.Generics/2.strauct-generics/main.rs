struct Pair<T,U> {
    first: T,
    second: U,
}

fn main() {
    let pair = Pair { first: 10, second: "Ten" };
    println!("first: {}, second: {}", pair.first, pair.second)
}