# Especificación: investigación integradora de rendimiento

**Estado:** draft  
**Issue:** #31

## Pregunta de investigación

Para una suma de `f32` independientes, ¿la variante explícita con cuatro lanes
de `wide` reduce el tiempo por operación frente a la referencia escalar en un
entorno y tamaños de entrada declarados, sin cambiar el resultado fuera de una
tolerancia de `1e-6`?

## Línea base, candidata y métricas

| Elemento | Decisión |
|---|---|
| Línea base | `sum_f32_scalar`, recorrido secuencial de un slice. |
| Candidata | `sum_f32_wide`, grupos de cuatro lanes y cola escalar. |
| Entradas | Vectores deterministas de 16, 4 096 y 1 048 576 valores `f32`. |
| Métrica primaria | Nanosegundos por suma, con menor valor como mejor. |
| Métricas de control | Igualdad aproximada del resultado, versión de Rust, arquitectura y modo de compilación. |
| Calentamiento | Separado de las muestras reportadas. |

## Invariantes

1. Ambas variantes reciben el mismo slice preconstruido; generar datos no forma
   parte de la medición.
2. Cada resultado SIMD se valida contra la referencia escalar antes de
   interpretar tiempos.
3. Se reportan al menos dos muestras por variante, así como mínimo y máximo; no
   se presenta una sola corrida como evidencia.
4. La conclusión nombra CPU, arquitectura, compilador, perfil y tamaño de
   entrada observados.
5. Una diferencia no concluyente es un resultado válido; no se modifica la
   implementación para perseguir una mejora sin una hipótesis nueva.

## Criterio de éxito

El estudio se considera reproducible cuando construye los tres tamaños de
entrada de forma determinista, valida su equivalencia numérica y entrega un
reporte por variante con muestras separadas del calentamiento. Solo puede
describir una mejora local si la candidata muestra una reducción consistente
para el tamaño y entorno reportados. No se fija un porcentaje de mejora: hacerlo
antes de medir convertiría la meta en sesgo de confirmación.

## Alternativas consideradas

| Alternativa | Decisión | Motivo y límite |
|---|---|---|
| Medir solo un tamaño grande | Rechazada | Oculta el costo fijo y no caracteriza la frontera de tamaño. |
| Usar una entrada aleatoria por muestra | Rechazada | Mezcla generación, cache y variación de entrada con el cálculo. |
| Tomar la mejor corrida | Rechazada | No comunica dispersión ni ruido. |
| Benchmark educativo con `Instant` | Adoptada | Es suficiente para enseñar contrato y reporte, no para inferencia estadística. |
| Profiling por hardware | Complementario | Puede explicar un hot path, pero depende de herramientas y permisos externos. |

## Amenazas a validez

La CPU puede variar frecuencia o migrar entre núcleos; el sistema puede tener
carga de fondo; el compilador puede auto-vectorizar la referencia; las
arquitecturas no comparten el mismo ancho de vector; y el orden de reducción de
punto flotante puede cambiar el redondeo. El reporte conserva estas amenazas y
no extrapola un resultado local a toda plataforma.

## Límites y decisión educativa

El estudio no cuenta instrucciones, cache misses ni asignaciones, y no promete
una mejora. Integra los contratos de medición, benchmark y SIMD del curso para
enseñar una decisión basada en evidencia. El modelo usa Rust estable, `wide`
autorizada y ningún bloque `unsafe`.
