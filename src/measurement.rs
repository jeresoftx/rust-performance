//! Contratos de datos para experimentos de rendimiento reproducibles.

/// Indica qué dirección representa una mejora para una métrica.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OptimizationDirection {
    /// Un valor menor representa una mejora, como el tiempo o las asignaciones.
    LowerIsBetter,
    /// Un valor mayor representa una mejora, como el throughput.
    HigherIsBetter,
}

/// Describe la métrica que un experimento observa.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Metric {
    name: String,
    unit: String,
    direction: OptimizationDirection,
}

impl Metric {
    /// Construye una métrica con nombre, unidad y dirección de mejora.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        unit: impl Into<String>,
        direction: OptimizationDirection,
    ) -> Self {
        Self {
            name: name.into(),
            unit: unit.into(),
            direction,
        }
    }

    /// Devuelve la unidad declarada para la observación.
    #[must_use]
    pub fn unit(&self) -> &str {
        &self.unit
    }
}

/// Datos que deben declararse antes de aceptar un experimento.
#[derive(Clone, Debug, PartialEq)]
pub struct ExperimentSpec {
    pub name: String,
    pub hypothesis: String,
    pub baseline: String,
    pub candidate: String,
    pub input: String,
    pub environment: String,
    pub metric: Metric,
    pub warmup_runs: u32,
    pub samples: Vec<f64>,
    pub threats_to_validity: Vec<String>,
}

/// Error de validación de un contrato de experimento.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentError {
    /// Falta un campo textual que evita interpretar la comparación.
    MissingField(&'static str),
    /// Una sola observación no permite contrastar ruido o variación.
    InsufficientSamples,
    /// Las muestras deben ser números finitos para ser interpretables.
    NonFiniteSample,
}

/// Experimento de rendimiento con contexto mínimo verificable.
#[derive(Clone, Debug, PartialEq)]
pub struct Experiment {
    spec: ExperimentSpec,
}

impl Experiment {
    /// Valida y conserva un experimento reproducible.
    pub fn new(spec: ExperimentSpec) -> Result<Self, ExperimentError> {
        for (value, field) in [
            (&spec.name, "name"),
            (&spec.hypothesis, "hypothesis"),
            (&spec.baseline, "baseline"),
            (&spec.candidate, "candidate"),
            (&spec.input, "input"),
            (&spec.environment, "environment"),
        ] {
            if value.trim().is_empty() {
                return Err(ExperimentError::MissingField(field));
            }
        }

        if spec.samples.len() < 2 {
            return Err(ExperimentError::InsufficientSamples);
        }

        if spec.samples.iter().any(|sample| !sample.is_finite()) {
            return Err(ExperimentError::NonFiniteSample);
        }

        Ok(Self { spec })
    }

    /// Devuelve el número de muestras observadas, sin incluir calentamiento.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.spec.samples.len()
    }

    /// Devuelve la métrica que se observó.
    #[must_use]
    pub fn metric(&self) -> &Metric {
        &self.spec.metric
    }

    /// Devuelve cuántas ejecuciones se reservaron para calentamiento.
    #[must_use]
    pub const fn warmup_runs(&self) -> u32 {
        self.spec.warmup_runs
    }
}
