fn compare<T: PartialOrd> (a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

fn main() {
    let a = 10;
    let b = 20;
    let smaller = compare(a, b);
    println!("The smaller number is: {}", smaller);

    let str1 = "Apple";
    let str2 = "Banana";
    let smaller_str = compare(str1, str2);
    println!("The Smaller string is: {}", smaller_str);
}