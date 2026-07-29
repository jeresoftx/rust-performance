# Especificación: caché y localidad

**Estado:** draft  
**Issue:** #11

## Concepto

La localidad espacial favorece datos próximos en memoria; la temporal favorece
reusar datos antes de que salgan de una caché. Ambas describen una relación
entre representación, patrón de acceso y jerarquía de memoria, no una mejora
garantizada por usar un contenedor particular.

## Problema

Dos algoritmos con complejidad asintótica semejante pueden tener costos muy
distintos si uno recorre memoria contigua y otro sigue referencias dispersas.
Sin embargo, una comparación pequeña o una sola arquitectura no demuestra que
la diferencia provenga exclusivamente de la caché.

## Invariantes de comparación

1. Los recorridos contiguo y disperso producen el mismo resultado observable.
2. Comparten longitud, distribución de datos, operaciones y configuración del
   benchmark.
3. La variante dispersa representa índices o referencias explícitos; no se
   inventan objetos para dramatizar el costo.
4. El reporte declara tamaño de entrada, arquitectura, perfil y límite de
   generalización.
5. El benchmark respalda una hipótesis sobre patrón de acceso; no mide tamaños
   pequeños como prueba de presión de caché.

## Alternativas

| Alternativa | Decisión | Límite |
|---|---|---|
| Decir que `Vec` siempre es más rápido | Rechazada | La carga y el algoritmo determinan el costo. |
| Simular una caché en el modelo | Fuera de alcance | Cambia el tema a microarquitectura detallada. |
| Comparar recorridos equivalentes | Adoptada | Ilustra patrón de acceso y conserva honestidad. |

## Modelo educativo

#12 implementará suma contigua de valores e indireccionamiento mediante una
permutación de índices. Validará que los índices estén dentro de límites y que
ambas variantes preserven la suma. Los benchmarks de #13 usarán varias longitudes
y describirán por qué los resultados pueden variar por máquina.

## Límites

No se controla tamaño de línea, asociatividad, prefetching, frecuencia de CPU
ni políticas de memoria del sistema. Los ejemplos enseñan a formular la
hipótesis, no a atribuir cada nanosegundo a un nivel de caché.
