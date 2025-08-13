// 간단한 vec! 비슷
macro_rules! my_vec {
    ( $( $x:expr ), * $(,)? ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    }
}

fn main() {
    let a = my_vec![1, 2, 3];
    println!("{:?}", a) // [1, 2, 3]
}