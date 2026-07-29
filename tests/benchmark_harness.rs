use rust_performance::benchmark::{BenchmarkConfig, BenchmarkError, BenchmarkRunner};

#[test]
fn rejects_a_benchmark_without_reported_samples() {
    assert_eq!(
        BenchmarkConfig::new("vacío", 0, 1, 0),
        Err(BenchmarkError::MissingSamples)
    );
}

#[test]
fn rejects_a_benchmark_without_iterations() {
    assert_eq!(
        BenchmarkConfig::new("vacío", 2, 0, 0),
        Err(BenchmarkError::MissingIterations)
    );
}

#[test]
fn keeps_warmup_out_of_reported_samples() {
    let config = BenchmarkConfig::new("contador", 3, 1, 2).expect("valid config");
    let mut calls = 0;

    let report = BenchmarkRunner::run(&config, || {
        calls += 1;
        calls
    });

    assert_eq!(report.sample_count(), 3);
    assert_eq!(calls, 5);
    assert!(report.minimum_ns() <= report.maximum_ns());
}
