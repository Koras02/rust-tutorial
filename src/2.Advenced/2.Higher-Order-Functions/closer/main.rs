fn apply_to_list<F>(list: &[i32], f: F) -> Vec<i32>
where 
    F: Fn(i32) -> i32,
    {
        list.iter().map(|&x| f(x)).collect()
    }

fn main() {
    let numbers = vec![1,2,3,4,5];
    let squared_numbers = apply_to_list(&numbers, |x| x * 2);
    println!("{:?}", squared_numbers); // Output: [2, 4, 6, 8, 10]
}