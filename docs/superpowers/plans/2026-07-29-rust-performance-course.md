# Plan de implementación de Rust Performance

**Estado:** activo · **Actualizado:** 2026-07-29 · **Repositorio:**
[`jeresoftx/rust-performance`](https://github.com/jeresoftx/rust-performance)

## Fuente de verdad

Este documento es la fuente operativa del curso. Su autoridad sigue este orden:

1. la petición directa de Joel para este repositorio;
2. RFC-0001 §2, §10, §13–§17 y §20;
3. RFC-0002 para trazabilidad issue–PR–Project;
4. `AGENTS.md` del repositorio;
5. los estándares de fuente de verdad, GitHub Delivery Workflow y bloque
   autónomo indicados por Joel en `v3.workspace/standards/docs/`.

El [GitHub Project #19](https://github.com/users/jeresoftx/projects/19) es la
representación operativa. Issues y PRs deben mantenerse alineados con este plan.

## Estado actual

- [x] Project #19 creado desde la plantilla oficial, vinculado al repositorio y
  con las vistas Backlog, Roadmap, Critical Path, In Progress, Review / PRs y
  Blocked.
- [x] Milestones, labels, 33 issues activos y sus campos operativos creados.
- [x] Backlog agrupado por `Milestone`; Roadmap y Critical Path usan layout de
  roadmap con fechas cargadas en todos los items.
- [~] #27 queda bloqueado solo para SIMD explícito que requiera `unsafe`,
  nightly o una dependencia externa; el resto del plan puede avanzar antes.

## Objetivo

Entregar un libro de ingeniería y crate educativo en Rust para diagnosticar,
medir y mejorar sistemas con evidencia reproducible. El curso cubre
benchmarking, profiling, caché, predicción de ramas, layout y asignación de
memoria, arenas seguras, zero-copy y SIMD. Toda conclusión declara hipótesis,
línea base, entradas, entorno y límites.

## Arquitectura educativa

Cada capítulo sigue RFC-0001 §2: concepto, problema, alternativas,
justificación e implementación. Cada tema se entrega en tres slices:
especificación, modelo Rust con pruebas y capítulo con ejemplos, ejercicios,
soluciones, diagrama y medición o declaración honesta de por qué no aplica.

El crate inicia sin dependencias y prohíbe `unsafe`. Criterion, un crate de
profiling o SIMD explícito necesitan decisión humana antes de incorporarse. El
capítulo SIMD puede medir auto-vectorización; código SIMD explícito queda
bloqueado hasta la autorización que se registre en #27.

## Criterio de cierre global

El plan termina cuando los issues activos #1–#34 estén cerrados por PRs
trazables; #19 está cerrado como duplicado y no pertenece al plan. Cada
PR tenga asignación, milestone, labels y Project; las validaciones aplicables
estén en verde; y los diez capítulos, ejemplos, soluciones, diagramas, ruta de
lectura y glosario estén en `draft`. Ningún capítulo se marca `reviewed` o
`published` sin revisión humana transversal.

## Milestones y roadmap estimado

Las fechas son pronósticos para Roadmap, no compromisos de publicación. Los
milestones no tienen fecha de vencimiento inventada.

| Milestone | Ventana estimada | Propósito |
|---|---|---|
| 0. Planeación y fundación | 2026-07-29 a 2026-08-01 | Fuente, Project, taxonomía y coordinación. |
| 1. Disciplina de medición | 2026-08-03 a 2026-08-21 | Evidencia, benchmark y profiling. |
| 2. Memoria y comportamiento de CPU | 2026-08-24 a 2026-09-18 | Caché, ramas, layout, asignaciones y arenas. |
| 3. Movimiento de datos y SIMD | 2026-09-21 a 2026-10-02 | Zero-copy, buffers y vectorización. |
| 4. Investigación integradora y cierre | 2026-10-05 a 2026-10-16 | Caso de estudio y coherencia editorial. |

## Ruta crítica

`#1 → #2 → #3 → #4 → #5 → #6 → #7 → #8 → #9 → #10 → #11 → #12 → #13 → #14 → #15 → #16 → #17 → #18 → #20 → #21 → #22 → #23 → #24 → #25 → #26 → #27 → #28 → #29 → #30 → #31 → #32 → #33 → #34`

La secuencia es pedagógica: cada capítulo reutiliza la disciplina de medición
del anterior. Se puede investigar en paralelo, pero no cerrar un issue antes de
su dependencia.

## Issue coordinador

- [x] #1 Coordinar plan, Project y trazabilidad de `rust-performance`.
  - Prioridad: P1. Estimación humana: 1d. Ruta crítica: sí.
  - Cierre: plan en `main`, Project enlazado, campos y vistas verificados,
    issues creados y cada item con fechas, prioridad, duración y dependencia.

## Milestone 1: Disciplina de medición

### Capítulo 01: evidencia y contrato de medición

- [x] #2 Especificar hipótesis, línea base, variables y amenazas a validez.
- [x] #3 Implementar y probar un modelo de experimento reproducible.
- [x] #4 Escribir capítulo, diagrama, ejemplos, ejercicios y soluciones.

### Capítulo 02: diseño de benchmarks reproducibles

- [x] #5 Especificar muestras, calentamiento, ruido y reporte de resultados.
- [x] #6 Implementar y probar un harness educativo sin dependencias externas.
- [x] #7 Escribir capítulo, ejemplos, ejercicios, soluciones y benchmark base.

### Capítulo 03: profiling y selección de hot paths

- [ ] #8 Especificar qué responde un perfil y qué no demuestra.
- [ ] #9 Implementar y probar un modelo de contadores e interpretación.
- [ ] #10 Escribir capítulo, diagrama, ejemplos, ejercicios y soluciones.

## Milestone 2: Memoria y comportamiento de CPU

### Capítulo 04: caché y locality

- [ ] #11 Especificar localidad espacial, temporal y condiciones de medición.
- [ ] #12 Implementar y probar modelos de recorrido contiguo y disperso.
- [ ] #13 Escribir capítulo, diagrama, ejemplos, ejercicios y benchmarks.

### Capítulo 05: ramas y layout orientado a datos

- [ ] #14 Especificar predicción de ramas, distribución y representación.
- [ ] #15 Implementar y probar modelos de ramas y layout alternativo.
- [ ] #16 Escribir capítulo, diagrama, ejemplos, ejercicios y benchmarks.

### Capítulo 06: asignaciones y costo de propiedad

- [ ] #17 Especificar asignación, reutilización, ownership y límites.
- [ ] #18 Implementar y probar modelos de construcción y reutilización.
- [ ] #20 Escribir capítulo, diagrama, ejemplos, ejercicios y benchmarks.

### Capítulo 07: arenas seguras y ciclos de vida

- [ ] #21 Especificar arena segura, capacidad, reset y alternativas.
- [ ] #22 Implementar y probar una arena educativa sin `unsafe`.
- [ ] #23 Escribir capítulo, diagrama, ejemplos, ejercicios y benchmarks.

## Milestone 3: Movimiento de datos y SIMD

### Capítulo 08: zero-copy, buffers y serialización

- [ ] #24 Especificar préstamos, buffers, parsing y costo de copias.
- [ ] #25 Implementar y probar parsing basado en slices y buffers.
- [ ] #26 Escribir capítulo, diagrama, ejemplos, ejercicios y benchmarks.

### Capítulo 09: SIMD y límites de vectorización

- [ ] #27 Decidir estrategia permitida para SIMD explícito y dependencias.
- [ ] #28 Especificar auto-vectorización, portabilidad y validación numérica.
- [ ] #29 Implementar y probar el modelo SIMD autorizado.
- [ ] #30 Escribir capítulo, diagrama, ejemplos, ejercicios y benchmarks.

## Milestone 4: Investigación integradora y cierre

### Capítulo 10: investigación completa de rendimiento

- [ ] #31 Especificar caso de estudio, métricas, línea base y criterio de éxito.
- [ ] #32 Implementar y probar una investigación reproducible de punta a punta.
- [ ] #33 Escribir capítulo, reporte, diagrama, ejercicios y soluciones.

### Cierre editorial

- [ ] #34 Completar ruta de lectura, glosario, referencias cruzadas y auditoría.

## Dependencias y blockers

| Issues | Dependencia | Bloqueador |
|---|---|---|
| #3, #6, #9, #12, #15, #18, #22, #25, #29, #32 | Especificación del capítulo | Ninguno adicional. |
| #4, #7, #10, #13, #16, #20, #23, #26, #30, #33 | Modelo del capítulo | Ninguno adicional. |
| #27 | #26 | Decisión humana solo si se solicita SIMD explícito, `unsafe` o crate externo. |
| #29 | #27 y #28 | No iniciar SIMD explícito sin autorización registrada. |
| #34 | #33 | Ninguno; no marca capítulos como revisados. |

## Contrato de issues y validación

Cada issue debe tener enlace a este plan, alcance, criterios de aceptación,
prioridad, estimación humana, fechas, dependencias, ruta crítica, blockers,
validación y definición de terminado. Todos se asignan a `jeresoftx`, tienen
milestone, labels y están agregados al Project #19.

| Tipo | Validación mínima |
|---|---|
| Especificación | Invariantes, alternativas, límites, enlaces y `git diff --check`. |
| Modelo | TDD, `cargo fmt --check`, Clippy, pruebas, doctests y diff limpio. |
| Capítulo | Modelo verde, ejemplos ejecutables, Mermaid, ejercicios, soluciones y benchmark o declaración honesta. |
| Decisión | Alternativas, impacto de dependencias/`unsafe` y autorización humana cuando aplique. |
| Cierre | Índice, enlaces, estados, glosario, ruta de lectura y suite completa. |

## Modo autónomo y siguiente bloque

Con autorización de Joel, cada slice usa `1 issue → 1 rama aislada → 1 commit
principal → 1 PR → checks → squash merge`. El PR debe agregarse y verificarse
en Project #19 conforme a RFC-0002; los items solo pasan a `Done` tras merge y
sincronización de `main`.

Siguiente bloque recomendado: `#2 → #3 → #4`, capítulo 01. No requiere
dependencias externas y establece la metodología del resto del curso.
