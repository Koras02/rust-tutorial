#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
  let dir = Direction::Up;

  match dir {
    Direction::Up => println!("Going Up!"),
    Direction::Down => println!("Going Down"),
    _ => println!("Other Direction")
  }
}
