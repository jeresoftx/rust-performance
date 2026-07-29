# Informe reproducible: suma escalar frente a SIMD

**Estado:** plantilla de evidencia local  
**Capítulo:** 10  
**Pregunta:** ¿La suma con `wide` reduce nanosegundos por operación frente a la
referencia escalar para los tamaños y entorno declarados?

## Entorno que se debe registrar

| Campo | Valor de esta ejecución |
|---|---|
| Fecha y hora | Pendiente de registrar. |
| CPU y arquitectura | Pendiente de registrar. |
| Sistema operativo | Pendiente de registrar. |
| Rust (`rustc -Vv`) | Pendiente de registrar. |
| Perfil y flags | `cargo run --release --example 10_investigacion` o valor real usado. |
| Carga de fondo relevante | Pendiente de registrar. |

## Contrato

- Línea base: `sum_f32_scalar`.
- Candidata: `sum_f32_wide` con `wide::f32x4` y cola escalar.
- Entradas: 16, 4 096 y 1 048 576 valores deterministas.
- Métrica: nanosegundos por suma; menor es mejor.
- Corrección: diferencia relativa no mayor a `1e-6`.
- Calentamiento: separado de las tres muestras reportadas por variante.

## Resultados por completar

| Tamaño | Escalar mínimo/máximo | SIMD mínimo/máximo | ¿Resultados equivalentes? | Observación |
|---:|---:|---:|---|---|
| 16 | Pendiente | Pendiente | Pendiente | Pendiente |
| 4 096 | Pendiente | Pendiente | Pendiente | Pendiente |
| 1 048 576 | Pendiente | Pendiente | Pendiente | Pendiente |

## Conclusión responsable

Completa esta sección solo después de registrar entorno y muestras. Una
diferencia observada describe este entorno y esta carga; no se extrapola a otra
CPU, arquitectura, distribución de datos o versión de compilador. Si la señal
no es consistente, la conclusión válida es que la hipótesis no se confirmó.

## Amenazas a validez

- Frecuencia dinámica, migración de núcleo y carga de fondo.
- Auto-vectorización posible de la línea base.
- Diferencias de ISA y ancho de vector entre arquitecturas.
- Redondeo de punto flotante por orden de reducción.
- Pocas muestras y ausencia de contadores de hardware.
