use rust_performance::measurement::{Experiment, ExperimentSpec, Metric, OptimizationDirection};

fn main() {
    let experiment = Experiment::new(ExperimentSpec {
        name: "Suma de enteros".into(),
        hypothesis: "Un acumulador reduce el tiempo medio para 10 000 enteros.".into(),
        baseline: "iterador".into(),
        candidate: "acumulador".into(),
        input: "10 000 enteros pseudoaleatorios; semilla 42".into(),
        environment: "Rust estable; release; arquitectura declarada".into(),
        metric: Metric::new("tiempo", "ns", OptimizationDirection::LowerIsBetter),
        warmup_runs: 3,
        samples: vec![102.0, 99.0, 101.0],
        threats_to_validity: vec!["frecuencia dinámica de CPU".into()],
    })
    .expect("el contrato está completo");

    println!(
        "{} muestras en {}",
        experiment.sample_count(),
        experiment.metric().unit()
    );
}
