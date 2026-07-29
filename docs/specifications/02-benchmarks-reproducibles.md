# Especificación: benchmarks reproducibles

**Estado:** draft  
**Issue:** #5  
**Capítulo relacionado:** 02, diseño de benchmarks reproducibles

## Concepto

Un benchmark es un procedimiento repetible para observar una operación bajo
una carga declarada. Su salida es una colección de muestras, no un número
aislado que pueda proclamarse como una propiedad absoluta del código.

## Problema

Las primeras ejecuciones pueden incluir compilación dinámica inexistente en
Rust, inicialización de datos, cachés frías o trabajo de asignación que no
representa la operación medida. Incluso después del calentamiento, el sistema
operativo, la frecuencia de CPU y otros procesos introducen ruido.

## Contrato del harness

El harness educativo de #6 recibirá una configuración con:

- `warmup_runs`: ejecuciones descartadas y separadas de las muestras.
- `sample_count`: número de muestras observadas, con mínimo de dos.
- `iterations_per_sample`: repeticiones de la operación dentro de cada muestra.
- `label`: identidad legible de la operación y de la carga.

Para cada muestra usará un reloj monotónico, medirá el lote completo y
normalizará a tiempo por iteración. Mantendrá el resultado de la operación
observable para evitar que un ejemplo trivial parezca trabajo útil aunque el
compilador pueda eliminarlo.

## Invariantes

1. Calentamiento y muestras reportadas no se mezclan.
2. Todas las muestras de una comparación usan la misma entrada y configuración.
3. `sample_count` es al menos dos e `iterations_per_sample` es mayor que cero.
4. El reporte conserva muestras, mínimo, máximo y media; no oculta la
   dispersión detrás de un único promedio.
5. El harness no declara significancia estadística ni causalidad.
6. Un benchmark compara implementaciones correctas; las pruebas de corrección
   viven fuera de la medición.

## Alternativas

| Alternativa | Decisión | Razón |
|---|---|---|
| Una ejecución de `Instant` | Rechazada | No separa calentamiento ni ruido. |
| Crate externo de benchmarks | Fuera de alcance | El capítulo enseña primero el procedimiento con biblioteca estándar. |
| Harness educativo con muestras | Adoptada | Hace visibles las decisiones y sus límites. |

## Reporte requerido

Todo ejemplo debe declarar entrada, entorno, configuración, unidad y amenaza a
validez. El resumen usa media, mínimo y máximo como descripción, no como prueba
de que una variante sea mejor en otra máquina. Un resultado debe incluir la
línea base y una variante; mostrar una sola serie sirve para caracterizar, no
para comparar.

## Límites

El harness no fija afinidad de CPU, no deshabilita turbo, no reemplaza análisis
estadístico y no garantiza que `Instant` tenga la misma resolución en todos los
sistemas. Es intencional: el estudiante debe aprender qué controla y qué no.
