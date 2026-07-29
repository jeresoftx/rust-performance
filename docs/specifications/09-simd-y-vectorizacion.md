# Especificación: SIMD y límites de vectorización

**Estado:** draft  
**Issue:** #28

## Concepto

SIMD aplica una misma operación a varios elementos mediante lanes. Puede venir
de auto-vectorización del compilador o de una representación explícita. En este
curso, `wide` permite mostrar la segunda sobre Rust estable sin exponer
`unsafe`.

## Invariantes

1. La variante SIMD se compara contra una referencia escalar del mismo cálculo.
2. El resultado incluye los elementos de cola que no llenan un vector completo.
3. Las comparaciones de punto flotante usan tolerancia declarada cuando aplica.
4. El modelo no asume que la CPU concreta tenga una aceleración universal.
5. La entrada, arquitectura, compilador y perfil se reportan con cualquier
   benchmark.

## Alternativas

| Alternativa | Decisión | Límite |
|---|---|---|
| Solo auto-vectorización | Complementaria | No muestra lanes explícitos ni su cola. |
| `std::simd` nightly | Rechazada | El curso compila en Rust estable. |
| Intrínsecos | Rechazada | Requieren `unsafe` y atan el ejemplo a plataforma. |
| `wide` | Adoptada | La aceleración sigue dependiendo de hardware y carga. |

## Modelo educativo

#29 sumará `f32` por grupos de cuatro con `wide::f32x4` y resolverá la cola de
forma escalar. Las pruebas contrastarán el resultado con la suma escalar en
longitudes múltiplo y no múltiplo de cuatro. No se presentará la diferencia de
precisión de punto flotante como error si está dentro de la tolerancia definida.

## Límites

Vectorizar puede empeorar una carga pequeña, aumentar presión de registros o no
ser seleccionado de igual manera por todas las arquitecturas. Los benchmarks
describen evidencia local, no una garantía de velocidad.
