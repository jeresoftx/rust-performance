use rust_performance::allocation::{build_message_fresh, build_message_reused};

fn main() {
    let mut buffer = Vec::with_capacity(32);
    println!("{:?}", build_message_fresh("hola"));
    println!("{:?}", build_message_reused(&mut buffer, "hola"));
}
