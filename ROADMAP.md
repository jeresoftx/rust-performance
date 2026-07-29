# ROADMAP

`rust-performance` estudia cómo medir y mejorar sistemas Rust sin confundir
correlación, microbenchmarks o intuición con evidencia. No tiene fechas límite:
registra dirección y calidad, no una carrera por terminar (RFC-0001 §1).

## Estado Actual

La fundación del repositorio está lista. Antes de crear capítulos o modelos se
debe elaborar un plan versionado y convertirlo en GitHub Project, milestones e
issues. Cada issue será asignado a `jeresoftx`, tendrá labels, milestone y
pertenecerá al Project; cada PR se agregará y verificará en el mismo Project,
conforme a RFC-0002.

## Dirección Técnica

1. Medición reproducible y diseño de experimentos.
2. Profiling y análisis de hot paths.
3. Caché, layout de memoria y asignaciones.
4. Zero-copy, buffers y serialización.
5. SIMD y evaluación de trade-offs.

## Fuera De Alcance Por Ahora

- Afirmar mejoras sin benchmark reproducible y contexto de carga.
- Usar `unsafe` o dependencias no triviales sin aprobación humana explícita.
- Duplicar el curso canónico de low-level o convertir el curso en una colección
  de trucos de microoptimización.
- Marcar contenido como revisado o publicado sin revisión humana.
