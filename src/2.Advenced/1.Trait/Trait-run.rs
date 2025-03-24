trait Speak {
    fn speak(&self);
}

struct Running;

impl Speak for Running {
    fn speak(&self) {
        println!("Run!");
    } 
}