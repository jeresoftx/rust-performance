use rust_performance::study::{run_sum_investigation, StudyError};

#[test]
fn reports_equivalent_variants_for_each_declared_input_size() {
    let report = run_sum_investigation(&[16, 4_096]).expect("valid investigation");

    assert_eq!(report.case_count(), 2);
    assert!(report.cases().iter().all(|case| case.results_match()));
    assert!(report
        .cases()
        .iter()
        .all(|case| case.baseline_samples() >= 2 && case.candidate_samples() >= 2));
}

#[test]
fn rejects_an_empty_or_zero_sized_input_plan() {
    assert_eq!(
        run_sum_investigation(&[]),
        Err(StudyError::MissingInputSizes)
    );
    assert_eq!(
        run_sum_investigation(&[16, 0]),
        Err(StudyError::ZeroSizedInput)
    );
}
