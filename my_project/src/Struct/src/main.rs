struct Person {
    name: String,
    age: u8
}

fn main() {
   let p = Person {
    name: String::from("Alice"),
    age: 30,
   };

   println!("{} is {} years old", p.name, p.age)
}
