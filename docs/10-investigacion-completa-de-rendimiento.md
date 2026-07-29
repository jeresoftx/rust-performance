# Investigación completa de rendimiento

**Estado:** draft

Una optimización responsable termina con una decisión explicable, aunque la
decisión sea conservar la línea base. Este caso reúne el contrato de evidencia,
el harness educativo y el modelo SIMD para investigar una sola pregunta sin
confundir tiempos locales con una promesa para todo hardware.

## Del problema al informe

```mermaid
flowchart LR
    Q[Pregunta e hipótesis] --> I[Entradas deterministas]
    I --> C[Corrección escalar vs SIMD]
    C --> W[Calentamiento y muestras]
    W --> R[Reporte de entorno y dispersión]
    R --> D[Decisión local con límites]
```

La referencia es `sum_f32_scalar`; la candidata, `sum_f32_wide`. Las entradas
de 16, 4 096 y 1 048 576 valores separan un caso pequeño de tamaños con más
trabajo. Antes de observar el reloj, ambas variantes deben producir resultados
equivalentes dentro de la tolerancia declarada.

```rust
use rust_performance::study::run_sum_investigation;

let report = run_sum_investigation(&[16, 4_096, 1_048_576])?;

for case in report.cases() {
    assert!(case.results_match());
    println!(
        "{} valores: {} muestras escalares, {} SIMD",
        case.input_len(),
        case.baseline_samples(),
        case.candidate_samples(),
    );
}
# Ok::<(), rust_performance::study::StudyError>(())
```

El ejemplo `10_investigacion` ejecuta ese reporte. Sus números no se guardan
como resultado canónico: cambian con CPU, compilador, flags, carga del sistema
y arquitectura. El [formato de informe](reports/10-investigacion-simd.md)
indica qué conservar junto con cualquier medición.

## Cómo interpretar el resultado

1. Comprueba que todos los casos conservan el resultado antes de comparar
   muestras.
2. Declara la versión de Rust, `--release`, CPU, arquitectura y carga de
   entrada.
3. Revisa mínimo y máximo por variante como dispersión descriptiva; no elijas
   solo la mejor corrida.
4. Si la diferencia no es consistente, conserva la variante más clara y anota
   que la hipótesis no quedó confirmada en ese entorno.
5. Si parece consistente, repite en el entorno objetivo y formula una nueva
   hipótesis antes de ampliar el cambio.

## Ejercicios

1. Explica por qué construir el `Vec<f32>` dentro de cada muestra invalidaría
   la pregunta que se investiga.
2. Registra una ejecución en modo debug y otra en `--release`; enumera qué
   comparación no es válida entre ellas.
3. Añade un tamaño de entrada que represente tu carga y actualiza el informe
   con sus amenazas a validez.
4. Diseña una investigación alternativa donde el resultado correcto no sea una
   suma; declara primero la referencia, candidata y observables.

## Soluciones orientativas

1. Medirías también asignación y construcción de datos, no solo la suma; la
   línea base y la candidata dejarían de responder la misma pregunta.
2. Cambian optimizaciones, inlining y layout; una diferencia no prueba el
   efecto de SIMD por sí sola.
3. Conserva distribución, longitud, semilla o construcción determinista,
   entorno, calentamiento, muestras y dispersión antes de comparar.
4. Un parser prestado frente a uno propietario puede ser válido si ambos
   aceptan el mismo formato y el reporte incluye copias, lifetimes y memoria.

## Límites

El harness usa `Instant` y resume pocas muestras; es una herramienta educativa,
no inferencia estadística ni profiling de hardware. El orden de suma de `f32`
puede redondear de forma distinta, por lo que la equivalencia usa tolerancia.
No se usa `unsafe`, nightly ni se afirma una mejora sin evidencia reproducible
en el entorno que importa.
