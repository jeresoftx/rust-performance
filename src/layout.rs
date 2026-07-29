//! Modelos de ramas y representaciones de campos calientes.

/// Registro con un campo caliente, uno frío y una etiqueta descriptiva.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record {
    hot: i64,
    cold: i64,
    label: String,
}

impl Record {
    /// Construye un registro para comparar layout de datos.
    #[must_use]
    pub fn new(hot: i64, cold: i64, label: impl Into<String>) -> Self {
        Self {
            hot,
            cold,
            label: label.into(),
        }
    }
}

/// Cuenta valores estrictamente positivos con semántica explícita.
#[must_use]
pub fn count_positive(values: &[i64]) -> usize {
    values.iter().filter(|&&value| value > 0).count()
}

/// Suma el campo caliente de una representación array-of-structs.
#[must_use]
pub fn sum_hot_fields_aos(records: &[Record]) -> i64 {
    records.iter().map(|record| record.hot).sum()
}

/// Suma el mismo campo caliente de una representación struct-of-arrays.
#[must_use]
pub fn sum_hot_fields_soa(hot_values: &[i64]) -> i64 {
    hot_values.iter().sum()
}
