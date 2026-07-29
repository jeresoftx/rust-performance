//! Suma SIMD explícita y segura con `wide`.

use wide::f32x4;

/// Suma con un recorrido escalar que sirve como referencia de corrección.
#[must_use]
pub fn sum_f32_scalar(values: &[f32]) -> f32 {
    values.iter().sum()
}

/// Suma por grupos de cuatro lanes y procesa la cola de forma escalar.
#[must_use]
pub fn sum_f32_wide(values: &[f32]) -> f32 {
    let mut lanes = f32x4::default();
    let mut chunks = values.chunks_exact(4);

    for chunk in &mut chunks {
        lanes += f32x4::new([chunk[0], chunk[1], chunk[2], chunk[3]]);
    }

    lanes.reduce_add() + chunks.remainder().iter().sum::<f32>()
}
