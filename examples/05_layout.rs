use rust_performance::layout::{count_positive, sum_hot_fields_aos, Record};

fn main() {
    let records = [Record::new(2, 10, "uno"), Record::new(3, 20, "dos")];
    println!("{}", count_positive(&[-3, 0, 2, 7]));
    println!("{}", sum_hot_fields_aos(&records));
}
