//! Modelos educativos para el curso de ingeniería de rendimiento en Rust.
//!
//! Las optimizaciones se estudian con una línea base, una hipótesis medible y
//! evidencia reproducible. Ningún resultado se considera publicado sin revisión
//! humana.

#![forbid(unsafe_code)]

pub mod allocation;
pub mod arena;
pub mod benchmark;
pub mod layout;
pub mod locality;
pub mod measurement;
pub mod profile;
pub mod simd;
pub mod zero_copy;

/// Devuelve la identidad del curso para comprobar la fundación del crate.
#[must_use]
pub const fn course_name() -> &'static str {
    "Rust Performance"
}

#[cfg(test)]
mod tests {
    use super::course_name;

    #[test]
    fn exposes_the_course_identity() {
        assert_eq!(course_name(), "Rust Performance");
    }
}
