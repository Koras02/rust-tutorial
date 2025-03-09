pub fn my_function() {
    println!("Hello from my_module");
}

pub mod outer {
  pub mod inner {
    pub fn inner_function() {
        println!("Hello from inner_function");
    }
  }
}