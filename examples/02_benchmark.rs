use rust_performance::benchmark::{BenchmarkConfig, BenchmarkRunner};

fn main() {
    let config =
        BenchmarkConfig::new("sumar 64 enteros", 5, 100, 2).expect("la configuración es válida");
    let report = BenchmarkRunner::run(&config, || (0_u64..64).sum::<u64>());

    println!(
        "{} muestras: {}..{} ns/iteración",
        report.sample_count(),
        report.minimum_ns(),
        report.maximum_ns()
    );
}
