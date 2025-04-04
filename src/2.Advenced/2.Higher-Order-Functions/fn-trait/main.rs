fn apply<F>(f: F, x: i32) -> i32
where 
    F: Fn(i32) -> i32,
    {
        f(x)
    }

fn main() {
    let square = |x| x * x;
    let result = apply(square, 5);
    println!("The square of 5 is: {}", result);  // Output: The square of 5 is: 25
}