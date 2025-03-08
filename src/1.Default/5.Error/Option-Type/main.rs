fn find_item(items: &[&str], search: &str) -> Option<usize> {
    for(index, &item) in items.iter().enumerate() {
        if item == search {
            return Some(index);
        }
    }
    None
}


fn main() {
    let items = ["사과", "바나나", "멜론"];

    match find_item(&items, "바나나") {
        Some(index) => println!("바나나의 인덱스: {}", index),
        None => println!("바나나를 찾을 수 없음"),
    }

   match find_item(&items, "포도") {
       Some(index) => println!("포도의 인덱스: {}", index),
       None => println!("포도를 찾을 수 없음"),
   }
}