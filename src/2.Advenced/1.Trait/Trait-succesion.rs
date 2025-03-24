trait Speak {
    fn speak(&self);
}

struct Human;


impl Speak for Human {
    fn speak(&self) {
        println!("Running!");
    }
}

fn human_speak<T: Speak>(human: T) {
    human.speak();
} 

fn main() {
    let human = Human;
    human_speak(human);
}