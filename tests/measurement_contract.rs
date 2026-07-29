use rust_performance::measurement::{
    Experiment, ExperimentError, ExperimentSpec, Metric, OptimizationDirection,
};

fn valid_spec() -> ExperimentSpec {
    ExperimentSpec {
        name: "Suma de muestras".into(),
        hypothesis: "La variante acumulada reduce el tiempo medio.".into(),
        baseline: "sumar con iterador".into(),
        candidate: "sumar con acumulador".into(),
        input: "10 000 enteros pseudoaleatorios; semilla 42".into(),
        environment: "Rust estable, perfil release, arquitectura declarada".into(),
        metric: Metric::new("tiempo", "ns", OptimizationDirection::LowerIsBetter),
        warmup_runs: 3,
        samples: vec![102.0, 99.0, 101.0],
        threats_to_validity: vec!["frecuencia dinámica de CPU".into()],
    }
}

#[test]
fn accepts_a_complete_reproducible_experiment() {
    let experiment = Experiment::new(valid_spec()).expect("valid experiment");

    assert_eq!(experiment.sample_count(), 3);
    assert_eq!(experiment.metric().unit(), "ns");
    assert_eq!(experiment.warmup_runs(), 3);
}

#[test]
fn rejects_a_missing_hypothesis() {
    let mut spec = valid_spec();
    spec.hypothesis.clear();

    assert_eq!(
        Experiment::new(spec),
        Err(ExperimentError::MissingField("hypothesis"))
    );
}

#[test]
fn rejects_an_experiment_with_a_single_sample() {
    let mut spec = valid_spec();
    spec.samples = vec![100.0];

    assert_eq!(
        Experiment::new(spec),
        Err(ExperimentError::InsufficientSamples)
    );
}

#[test]
fn rejects_non_finite_measurements() {
    let mut spec = valid_spec();
    spec.samples[1] = f64::NAN;

    assert_eq!(Experiment::new(spec), Err(ExperimentError::NonFiniteSample));
}
