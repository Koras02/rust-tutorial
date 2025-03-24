trait Speak {
    fn speak(&self) {
        println!("Some sound");
    }
}

struct Bird;

impl Speak for Bird {
    fn speak(&self) {
        println!("Tweet!");
    }
}

 
fn main() {
    let bird = Bird;
    bird.speak();
}