fn make_multiplier(factor: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x * factor)
}

fn main() {
    let times_two = make_multiplier(2);
    let result = times_two(5);
    println!("5 times 2 is {}", result); // Output: 5 times 2 is 10

}