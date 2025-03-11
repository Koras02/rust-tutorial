struct Pair <T, U> {
    first: T,
    second: U,
}

fn main() {
   let integer_pair = Pair { first: 10, second: 20};
   let string_pair = Pair { first: "Apple", second: "Banana"};

   println!("The integer pair is: ({}, {})", integer_pair.first, integer_pair.second);
   println!("The string pair is: ({}, {})", string_pair.first, string_pair.second);
}