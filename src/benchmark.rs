//! Harness educativo para recolectar muestras de duración.

use std::hint::black_box;
use std::time::Instant;

/// Configuración explícita para un benchmark educativo.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BenchmarkConfig {
    label: String,
    sample_count: usize,
    iterations_per_sample: usize,
    warmup_runs: usize,
}

impl BenchmarkConfig {
    /// Crea una configuración que separa calentamiento y muestras observadas.
    pub fn new(
        label: impl Into<String>,
        sample_count: usize,
        iterations_per_sample: usize,
        warmup_runs: usize,
    ) -> Result<Self, BenchmarkError> {
        if sample_count < 2 {
            return Err(BenchmarkError::MissingSamples);
        }
        if iterations_per_sample == 0 {
            return Err(BenchmarkError::MissingIterations);
        }

        Ok(Self {
            label: label.into(),
            sample_count,
            iterations_per_sample,
            warmup_runs,
        })
    }
}

/// Errores de una configuración que no puede producir evidencia útil.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BenchmarkError {
    /// Menos de dos muestras no permite describir variación.
    MissingSamples,
    /// Una muestra sin trabajo no puede normalizarse por iteración.
    MissingIterations,
}

/// Resultado descriptivo de un benchmark, expresado en nanosegundos por iteración.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BenchmarkReport {
    label: String,
    samples_ns: Vec<u128>,
}

impl BenchmarkReport {
    /// Devuelve el número de muestras observadas, sin calentamiento.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.samples_ns.len()
    }

    /// Devuelve la muestra mínima en nanosegundos por iteración.
    #[must_use]
    pub fn minimum_ns(&self) -> u128 {
        *self
            .samples_ns
            .iter()
            .min()
            .expect("report always has samples")
    }

    /// Devuelve la muestra máxima en nanosegundos por iteración.
    #[must_use]
    pub fn maximum_ns(&self) -> u128 {
        *self
            .samples_ns
            .iter()
            .max()
            .expect("report always has samples")
    }
}

/// Ejecuta un procedimiento de medición; no interpreta causalidad ni significancia.
pub struct BenchmarkRunner;

impl BenchmarkRunner {
    /// Ejecuta calentamiento y recolecta muestras normalizadas por iteración.
    #[must_use]
    pub fn run<T>(config: &BenchmarkConfig, mut operation: impl FnMut() -> T) -> BenchmarkReport {
        for _ in 0..config.warmup_runs {
            black_box(operation());
        }

        let mut samples_ns = Vec::with_capacity(config.sample_count);
        for _ in 0..config.sample_count {
            let start = Instant::now();
            for _ in 0..config.iterations_per_sample {
                black_box(operation());
            }
            let elapsed_ns = start.elapsed().as_nanos();
            samples_ns.push(elapsed_ns / config.iterations_per_sample as u128);
        }

        BenchmarkReport {
            label: config.label.clone(),
            samples_ns,
        }
    }
}
