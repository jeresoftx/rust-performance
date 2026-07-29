//! Investigación reproducible que integra medición, benchmarks y SIMD.

use crate::benchmark::{BenchmarkConfig, BenchmarkRunner};
use crate::simd::{sum_f32_scalar, sum_f32_wide};

const SAMPLE_COUNT: usize = 3;
const WARMUP_RUNS: usize = 1;
const RELATIVE_TOLERANCE: f32 = 1e-6;

/// Error para un plan de entradas que no puede representar una investigación.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StudyError {
    /// No hay tamaños de entrada que comparar.
    MissingInputSizes,
    /// Una entrada vacía no ejercita el cálculo que se estudia.
    ZeroSizedInput,
}

/// Resumen de muestras de una variante, sin convertirlas en una conclusión causal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VariantSummary {
    sample_count: usize,
    minimum_ns: u128,
    maximum_ns: u128,
}

impl VariantSummary {
    /// Devuelve cuántas muestras reportadas contiene la variante.
    #[must_use]
    pub const fn sample_count(&self) -> usize {
        self.sample_count
    }

    /// Devuelve la muestra mínima normalizada en nanosegundos.
    #[must_use]
    pub const fn minimum_ns(&self) -> u128 {
        self.minimum_ns
    }

    /// Devuelve la muestra máxima normalizada en nanosegundos.
    #[must_use]
    pub const fn maximum_ns(&self) -> u128 {
        self.maximum_ns
    }
}

/// Resultado verificable de un tamaño de entrada del caso de estudio.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StudyCase {
    input_len: usize,
    baseline_samples: VariantSummary,
    candidate_samples: VariantSummary,
    results_match: bool,
}

impl StudyCase {
    /// Devuelve el número de valores usados por este caso.
    #[must_use]
    pub const fn input_len(&self) -> usize {
        self.input_len
    }

    /// Devuelve el resumen de la referencia escalar.
    #[must_use]
    pub const fn baseline_samples(&self) -> usize {
        self.baseline_samples.sample_count()
    }

    /// Devuelve el resumen de la variante SIMD.
    #[must_use]
    pub const fn candidate_samples(&self) -> usize {
        self.candidate_samples.sample_count()
    }

    /// Indica si ambos cálculos respetaron la tolerancia declarada.
    #[must_use]
    pub const fn results_match(&self) -> bool {
        self.results_match
    }
}

/// Reporte de una investigación que aún debe interpretarse en su entorno.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvestigationReport {
    cases: Vec<StudyCase>,
}

impl InvestigationReport {
    /// Devuelve los casos en el mismo orden que el plan de entradas.
    #[must_use]
    pub fn cases(&self) -> &[StudyCase] {
        &self.cases
    }

    /// Devuelve cuántos tamaños de entrada se investigaron.
    #[must_use]
    pub fn case_count(&self) -> usize {
        self.cases.len()
    }
}

/// Ejecuta el caso de estudio para cada tamaño declarado.
///
/// Los datos se construyen antes de medir, el calentamiento queda fuera de las
/// muestras y cada candidata se valida contra la referencia escalar. Los tiempos
/// son evidencia local: el reporte no declara una variante ganadora.
pub fn run_sum_investigation(input_sizes: &[usize]) -> Result<InvestigationReport, StudyError> {
    if input_sizes.is_empty() {
        return Err(StudyError::MissingInputSizes);
    }
    if input_sizes.contains(&0) {
        return Err(StudyError::ZeroSizedInput);
    }

    let cases = input_sizes
        .iter()
        .copied()
        .map(run_case)
        .collect::<Vec<_>>();

    Ok(InvestigationReport { cases })
}

fn run_case(input_len: usize) -> StudyCase {
    let values = deterministic_values(input_len);
    let baseline = sum_f32_scalar(&values);
    let candidate = sum_f32_wide(&values);
    let config = BenchmarkConfig::new("suma", SAMPLE_COUNT, 1, WARMUP_RUNS)
        .expect("fixed study configuration is valid");
    let scalar_report = BenchmarkRunner::run(&config, || sum_f32_scalar(&values));
    let wide_report = BenchmarkRunner::run(&config, || sum_f32_wide(&values));

    StudyCase {
        input_len,
        baseline_samples: VariantSummary {
            sample_count: scalar_report.sample_count(),
            minimum_ns: scalar_report.minimum_ns(),
            maximum_ns: scalar_report.maximum_ns(),
        },
        candidate_samples: VariantSummary {
            sample_count: wide_report.sample_count(),
            minimum_ns: wide_report.minimum_ns(),
            maximum_ns: wide_report.maximum_ns(),
        },
        results_match: approximately_equal(baseline, candidate),
    }
}

fn deterministic_values(input_len: usize) -> Vec<f32> {
    (0..input_len)
        .map(|index| (index % 16 + 1) as f32 / 16.0)
        .collect()
}

fn approximately_equal(left: f32, right: f32) -> bool {
    (left - right).abs() <= RELATIVE_TOLERANCE * left.abs().max(right.abs()).max(1.0)
}
