use rust_performance::zero_copy::{parse_borrowed, parse_owned};

fn main() {
    let input = "lang=rust;mode=release";
    println!("{:?}", parse_borrowed(input));
    println!("{:?}", parse_owned(input));
}
