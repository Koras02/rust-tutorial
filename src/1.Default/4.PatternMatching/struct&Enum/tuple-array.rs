fn main() {
    let tuple = (1, 2, 3);

    match tuple {
        (1, y, z) => println!("First is 1, y: {}, z: {}", y, z),
        (0, _, _) => println!("First is 0"),
        _ => println!("Something else"),
    }

    let array = [1, 2, 3];

    match array {
        [1, ..] => println!("Array starts with 1"),
        _ => println!("Something else"),
    }
}
