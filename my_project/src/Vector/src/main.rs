fn main() {
    let mut nums = vec![1,2,3];
    nums.push(4);
    for n in &nums {
        println!("{}", n)
    }
}