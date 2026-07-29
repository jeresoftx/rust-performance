use rust_performance::simd::{sum_f32_scalar, sum_f32_wide};

#[test]
fn wide_sum_matches_scalar_for_complete_lanes() {
    let values = [1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

    assert!((sum_f32_wide(&values) - sum_f32_scalar(&values)).abs() < 1e-6);
}

#[test]
fn wide_sum_processes_a_scalar_tail() {
    let values = [1.0_f32, 2.0, 3.0, 4.0, 5.0];

    assert!((sum_f32_wide(&values) - sum_f32_scalar(&values)).abs() < 1e-6);
}
