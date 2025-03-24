trait Speak {
    fn speak(&self);
}

struct Running;

impl Speak for Running {
    fn speak(&self) {
        println!("Run!");
    } 
}

fn let_speak(s: &dyn Speak) {
    s.speak();
}

fn main() {
    let run = Running;
    let_speak(&run);
}