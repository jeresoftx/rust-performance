use rust_performance::locality::{sum_contiguous, sum_indirected, LocalityError};

#[test]
fn equivalent_contiguous_and_indirected_traversals_preserve_the_sum() {
    let values = [4_i64, -2, 7, 1];
    let order = [2, 0, 3, 1];

    assert_eq!(sum_contiguous(&values), 10);
    assert_eq!(sum_indirected(&values, &order), Ok(10));
}

#[test]
fn rejects_an_out_of_bounds_indirection() {
    assert_eq!(
        sum_indirected(&[1_i64, 2], &[0, 2]),
        Err(LocalityError::InvalidIndex {
            index: 2,
            length: 2
        })
    );
}
