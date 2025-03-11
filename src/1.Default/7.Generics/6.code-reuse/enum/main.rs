enum Option<T> {
    On(T),
    None,
}

fn main() {
    let some_bool = Option::On(true);
    let _none_bool: Option<i32> = Option::None; 

   // Option example
   match some_bool {
     Option::On(value) => println!("Power is on: {}", value),
     Option::None => println!("Power is off"),
   }
}