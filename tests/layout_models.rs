use rust_performance::layout::{count_positive, sum_hot_fields_aos, sum_hot_fields_soa, Record};

#[test]
fn conditional_count_preserves_the_population_semantics() {
    assert_eq!(count_positive(&[-3, 0, 2, 7, -1]), 2);
}

#[test]
fn aos_and_soa_hot_field_layouts_produce_the_same_sum() {
    let records = [
        Record::new(2, 10, "uno"),
        Record::new(3, 20, "dos"),
        Record::new(-1, 30, "tres"),
    ];
    let hot = [2, 3, -1];

    assert_eq!(sum_hot_fields_aos(&records), 4);
    assert_eq!(sum_hot_fields_soa(&hot), 4);
}
