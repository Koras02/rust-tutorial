trait Container {
    type Item;

    fn get(&self) -> Self::Item;
}

struct MyContainer;

impl Container for MyContainer {
    type Item = i32;

    fn get(&self) -> Self::Item {
        42
    }
}

fn main() {
    let container = MyContainer;

    // get 메서드 호출 
    let value: i32 = container.get(); // 연관 타입으로 i32 반환
    println!("Value: {}", value); // "Value: 42" 
}