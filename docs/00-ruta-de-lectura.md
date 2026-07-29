# Ruta de lectura

**Estado del curso:** draft completo; pendiente de revisión humana transversal.

Este libro se recorre en orden porque cada capítulo reutiliza el contrato de
evidencia del anterior. No es una lista de trucos para acelerar código: es una
práctica para decidir qué medir, qué representa una mejora y qué límite debe
acompañar una conclusión.

## Recorrido recomendado

1. **[Evidencia y contrato de medición](01-evidencia-y-contrato-de-medicion.md):**
   empieza por hipótesis, línea base y amenazas a validez.
2. **[Benchmarks reproducibles](02-benchmarks-reproducibles.md)** y
   **[Profiling y hot paths](03-profiling-y-hot-paths.md):** aprende a producir
   muestras y a convertir una observación en una pregunta medible.
3. **[Caché y localidad](04-cache-y-localidad.md)**,
   **[Ramas y layout orientado a datos](05-ramas-y-layout.md)** y
   **[Asignaciones y costo de propiedad](06-asignaciones-y-propiedad.md):**
   relaciona representación, CPU y trabajo de memoria.
4. **[Arenas seguras y ciclos de vida](07-arenas-seguras-y-ciclos-de-vida.md)**
   y **[Zero-copy, buffers y serialización](08-zero-copy-y-buffers.md):**
   estudia ownership y movimiento de datos sin ocultar sus contratos.
5. **[SIMD y límites de vectorización](09-simd-y-limites-de-vectorizacion.md):**
   compara lanes explícitos con una referencia escalar y con límites de
   portabilidad.
6. **[Investigación completa de rendimiento](10-investigacion-completa-de-rendimiento.md):**
   integra los pasos en un informe de evidencia local.

## Atajos conscientes

- Para diseñar un benchmark nuevo, vuelve a los capítulos 01 y 02 antes de
  reutilizar un ejemplo de memoria o SIMD.
- Para diagnosticar una ruta observada, usa el capítulo 03 y formula después
  una hipótesis que el capítulo 02 pueda contrastar.
- Para cambiar representación o ownership, compara capítulos 04 a 08 y
  conserva los mismos resultados observables antes de medir tiempos.
- Para comunicar resultados, usa el capítulo 10 y su
  [plantilla de informe](reports/10-investigacion-simd.md); un resultado local
  no cambia el estado `draft` del curso.

Consulta el [glosario](11-glosario.md) cuando un término de medición,
arquitectura u ownership aparezca por primera vez.
