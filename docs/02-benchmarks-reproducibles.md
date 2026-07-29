# Benchmarks reproducibles

**Estado:** draft

Medir rápido no equivale a medir bien. Un benchmark útil describe qué trabajo
ejecutó, cuántas veces lo hizo y qué condiciones pueden explicar sus muestras.

## Concepto y problema

Cada muestra contiene variación. El calentamiento evita mezclar ejecuciones de
preparación con el reporte, pero no elimina el ruido del sistema operativo, la
frecuencia dinámica de CPU o diferencias de microarquitectura. Por eso el
harness conserva una serie y reporta mínimo, máximo y media descriptiva.

## Procedimiento

```mermaid
flowchart LR
    C[Configurar entrada y operación] --> W[Calentar y descartar]
    W --> S[Tomar varias muestras]
    S --> N[Normalizar por iteración]
    N --> R[Reportar serie y límites]
    R --> Q[Comparar solo implementaciones correctas]
```

`BenchmarkConfig` exige al menos dos muestras y una o más iteraciones. El
`BenchmarkRunner` usa `std::hint::black_box` para mantener observable el valor
devuelto por la operación. Esto reduce un riesgo de ejemplos triviales; no
convierte el resultado en una prueba de causalidad.

## Ejemplo

```rust
use rust_performance::benchmark::{BenchmarkConfig, BenchmarkRunner};

let config = BenchmarkConfig::new("sumar 64 enteros", 5, 100, 2)?;
let report = BenchmarkRunner::run(&config, || (0_u64..64).sum::<u64>());

assert_eq!(report.sample_count(), 5);
println!("{}..{} ns/iteración", report.minimum_ns(), report.maximum_ns());
# Ok::<(), rust_performance::benchmark::BenchmarkError>(())
```

El ejemplo no afirma que la suma tenga esa duración en otra máquina. Para una
comparación, usa la misma entrada, configuración y condición de corrección para
la línea base y la variante candidata.

## Lectura responsable

1. Registra Rust, perfil, arquitectura, carga y entrada antes de observar el
   resultado.
2. No elijas una sola muestra favorable: conserva mínimo, máximo y media.
3. Cambia una variable por comparación cuando sea posible.
4. Repite en el entorno que representa la decisión; una laptop no describe
   automáticamente producción.
5. Usa profiling cuando no sepas dónde está el trabajo relevante.

## Ejercicios

1. Configura un benchmark con 10 muestras, 1 000 iteraciones y 3 calentamientos.
2. Explica por qué una serie con mínimo 10 ns y máximo 90 ns merece
   investigación antes de optimizar.
3. Diseña la entrada para comparar `Vec::push` con y sin capacidad reservada.
4. Indica qué resultado observable debe conservar un benchmark de serialización.

## Soluciones orientativas

1. `BenchmarkConfig::new("operación", 10, 1_000, 3)`; todavía falta declarar
   entrada y entorno fuera de la configuración.
2. La dispersión puede provenir de ruido o de una carga irregular; el promedio
   solo no explica la causa.
3. Conserva longitud, distribución y datos; mide la construcción, no generar
   datos distintos en cada variante.
4. La representación decodificada o los bytes emitidos deben coincidir antes de
   comparar tiempo o asignaciones.

## Límites

El harness no reemplaza Criterion, control de afinidad, estadísticas robustas o
un perfil. Enseña qué información no se debe esconder antes de elegir una
herramienta más especializada.
