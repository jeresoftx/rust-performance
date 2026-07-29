use rust_performance::locality::{sum_contiguous, sum_indirected};

fn main() {
    let values = [4_i64, -2, 7, 1];
    let order = [2, 0, 3, 1];

    println!("{}", sum_contiguous(&values));
    println!("{:?}", sum_indirected(&values, &order));
}
