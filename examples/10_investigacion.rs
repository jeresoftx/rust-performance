use rust_performance::study::run_sum_investigation;

fn main() {
    let report = run_sum_investigation(&[16, 4_096, 1_048_576]).expect("valid study");

    for case in report.cases() {
        println!(
            "{} valores: equivalencia={}, escalar={} muestras, SIMD={} muestras",
            case.input_len(),
            case.results_match(),
            case.baseline_samples(),
            case.candidate_samples(),
        );
    }
}
