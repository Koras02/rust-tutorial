use my_macro::MyTrait;

trait MyTrait {
    fn hello();
}

#[derive(MyTrait)]
struct MyStruct;

fn main() {
    MyStruct::hello();
}