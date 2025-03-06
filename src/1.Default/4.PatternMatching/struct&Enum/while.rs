fn main() {
  let mut optional_values = vec![Some(1), Some(2), None, Some(3)];

  while let Some(value) = optional_values.pop() {
    match value {
        Some(v) => println!("Value: {}", v),
        None => println!("No Value"),
    }
  }   
}