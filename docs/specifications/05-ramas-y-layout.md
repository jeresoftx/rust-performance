# Especificación: ramas y layout orientado a datos

**Estado:** draft  
**Issue:** #14

## Concepto

Una rama condicional depende de una distribución de datos y de la predicción
del procesador. El layout define qué campos se cargan y recorren juntos. Ambos
afectan rendimiento, pero sus efectos se deben comparar con semántica y entrada
equivalentes.

## Problema

Reescribir una condición o separar estructuras por campos puede reducir trabajo
en una carga y aumentar complejidad, memoria o costo de mantenimiento en otra.
No basta con llamar "branchless" o "data-oriented" a una variante para que sea
mejor.

## Invariantes

1. Las variantes cuentan o filtran la misma población.
2. La distribución de datos se declara: sesgada, alternada o pseudoaleatoria.
3. El modelo separa representación de comparación: no usa resultados de
   benchmark para decidir corrección.
4. Un layout alternativo conserva los campos observables requeridos por la
   operación.
5. La medición declara tamaño y arquitectura; no atribuye cambios únicamente a
   branch prediction o caché.

## Alternativas

| Alternativa | Decisión | Límite |
|---|---|---|
| Convertir todo a ramas eliminadas | Rechazada | Puede reducir claridad o empeorar otra carga. |
| Usar AoS o SoA por moda | Rechazada | La operación define qué campos se necesitan juntos. |
| Comparar operación y distribución explícitas | Adoptada | Permite justificar el trade-off. |

## Modelo educativo

#15 implementará conteo condicional y suma de campos calientes en dos layouts.
Las pruebas demostrarán equivalencia de resultado; los benchmarks de #16
compararán distribuciones declaradas y describirán los límites.

## Límites

El modelo no mide fallos de predicción ni contadores de hardware. Una diferencia
de tiempo es una señal para investigar con herramientas de plataforma, no una
prueba de su causa microarquitectónica.
