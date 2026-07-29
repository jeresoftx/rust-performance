//! Modelos de recorridos equivalentes con patrones de acceso distintos.

/// Error al construir un recorrido por indireccionamiento.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalityError {
    /// Un índice no apunta a un valor disponible.
    InvalidIndex { index: usize, length: usize },
}

/// Suma valores en el orden en que viven contiguamente en el slice.
#[must_use]
pub fn sum_contiguous(values: &[i64]) -> i64 {
    values.iter().sum()
}

/// Suma los mismos valores siguiendo un orden explícito de índices.
pub fn sum_indirected(values: &[i64], order: &[usize]) -> Result<i64, LocalityError> {
    let mut total = 0;
    for &index in order {
        let value = values.get(index).ok_or(LocalityError::InvalidIndex {
            index,
            length: values.len(),
        })?;
        total += value;
    }
    Ok(total)
}
