//! Modelos educativos para el curso de ingeniería de rendimiento en Rust.
//!
//! Las optimizaciones se estudian con una línea base, una hipótesis medible y
//! evidencia reproducible. Ningún resultado se considera publicado sin revisión
//! humana.

#![forbid(unsafe_code)]

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
