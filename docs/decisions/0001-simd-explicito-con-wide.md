# Decisión 0001: SIMD explícito con `wide`

**Estado:** aceptada el 2026-07-29  
**Issue:** [#27](https://github.com/jeresoftx/rust-performance/issues/27)  
**Ámbito:** capítulo 09, SIMD y límites de vectorización

## Contexto

El curso necesita mostrar SIMD explícito sin abandonar Rust estable ni la
prohibición de `unsafe` declarada por el crate. La auto-vectorización continúa
siendo parte del capítulo, pero no permite explicar por sí sola la
representación de varias operaciones por instrucción.

## Decisión

Se incorpora `wide` `1.5.0` como única dependencia directa para las lecciones
de SIMD explícito. La autorización humana prohíbe `unsafe`, nightly y otras
dependencias directas para este propósito.

## Alternativas consideradas

- **Solo auto-vectorización:** se conserva para comparar el trabajo del
  compilador, pero no cubre el modelo explícito.
- **`std::simd` o `portable_simd`:** descartado porque requiere nightly.
- **Intrínsecos de arquitectura:** descartados porque requieren `unsafe` y
  atan el ejemplo a una plataforma.
- **Despacho dinámico adicional:** fuera de alcance para una primera lección;
  introduciría una segunda preocupación antes del modelo SIMD.

## Consecuencias

- Los ejemplos posteriores usarán solamente la API pública y segura de
  `wide`.
- Cada ejemplo SIMD tendrá una implementación escalar de referencia y pruebas
  de equivalencia numérica adecuadas para su dominio.
- Los benchmarks declararán arquitectura, compilador, perfil, entrada y
  límites; no prometerán aceleración universal.
- Esta decisión no autoriza otras dependencias externas, nightly ni `unsafe`.
