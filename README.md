# Rust Performance

Curso técnico complementario de Jeresoft Academy para estudiar ingeniería de
rendimiento en Rust. Explica cómo construir una línea base, formular una
hipótesis, medirla de forma reproducible y decidir si una optimización conserva
la claridad y la corrección del sistema.

El objetivo no es perseguir microsegundos sin contexto. Antes de modificar una
asignación, un acceso a memoria o una rama, el curso pregunta qué problema
observable existe, qué alternativa lo explica y qué medición podría refutar la
hipótesis (RFC-0001 §2 y §10).

## Lugar En El Camino

Este curso complementa las secciones de benchmarks de los demás repositorios de
Jeresoft Academy. Conecta Rust, sistemas operativos, estructuras de datos y
programación de bajo nivel, sin sustituir los cursos canónicos de concurrencia,
algoritmos o low-level.

**Nivel:** intermedio-avanzado. **Prerequisitos:** Rust básico, ownership,
complejidad algorítmica y nociones de arquitectura de computadores.

## Temas Planeados

- Diseño de benchmarks y líneas base reproducibles.
- Profiling y lectura crítica de perfiles.
- Jerarquía de caché, locality y branch prediction.
- Layout de memoria, asignación y arena allocators.
- Zero-copy, buffers y costo de serialización.
- SIMD y límites de la vectorización.

Los capítulos, su orden y sus criterios de aceptación viven en el
[plan versionado](docs/superpowers/plans/2026-07-29-rust-performance-course.md).

## Estructura

```text
docs/       Capítulos compatibles con mdBook.
src/        Modelos educativos en Rust.
examples/   Ejemplos: básico, intermedio, avanzado y caso real.
tests/      Pruebas de integración.
benches/    Mediciones reproducibles y su metodología.
diagrams/   Diagramas Mermaid.
assets/     Fuentes de visualizaciones.
```

## Verificación

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

## Gobernanza

- La estructura sigue RFC-0001 §15; los capítulos seguirán §14 y §16.
- Antes de iniciar desarrollo se crearán plan, GitHub Project, milestones e
  issues asignados a `jeresoftx`.
- El avance operativo vive en el [GitHub Project](https://github.com/users/jeresoftx/projects/19).
- Cada PR deberá pertenecer al mismo GitHub Project que su issue, conforme a
  RFC-0002.
- El código usa `MIT OR Apache-2.0`; el contenido educativo usa `CC BY-SA 4.0`.
- Ningún capítulo se marcará como `reviewed` o `published` sin revisión humana.
