# Glosario

**Estado:** draft

| Término | Definición operativa | Primera lectura |
|---|---|---|
| Amenaza a validez | Condición que puede explicar una diferencia observada sin que la candidata sea la causa. | [01](01-evidencia-y-contrato-de-medicion.md) |
| Benchmark | Procedimiento repetible que recolecta muestras para responder una pregunta acotada. | [02](02-benchmarks-reproducibles.md) |
| Calentamiento | Ejecuciones previas que no se incluyen en las muestras reportadas. | [02](02-benchmarks-reproducibles.md) |
| Hot path | Ruta donde un perfil observó trabajo relevante bajo una carga concreta. | [03](03-profiling-y-hot-paths.md) |
| Línea base | Implementación o estado contra el cual se contrasta una candidata. | [01](01-evidencia-y-contrato-de-medicion.md) |
| Localidad | Probabilidad de reutilizar datos cercanos en tiempo o posición de memoria. | [04](04-cache-y-localidad.md) |
| Layout | Organización física de campos y colecciones que condiciona los accesos. | [05](05-ramas-y-layout.md) |
| Asignación | Obtención de memoria para un valor; su costo depende de carga, allocator y vida útil. | [06](06-asignaciones-y-propiedad.md) |
| Arena segura | Colección que comparte ciclo de vida y usa handles verificables sin punteros crudos. | [07](07-arenas-seguras-y-ciclos-de-vida.md) |
| Zero-copy | Interfaz que reutiliza un buffer mediante préstamos en vez de copiar cada campo. | [08](08-zero-copy-y-buffers.md) |
| SIMD | Operación aplicada a varios datos independientes mediante lanes. | [09](09-simd-y-limites-de-vectorizacion.md) |
| Cola | Elementos que no llenan un grupo SIMD y requieren un recorrido complementario. | [09](09-simd-y-limites-de-vectorizacion.md) |
| Evidencia local | Resultado válido para la carga y entorno declarados, sin garantía de generalización. | [10](10-investigacion-completa-de-rendimiento.md) |

El glosario facilita la lectura, pero no reemplaza los invariantes ni límites de
cada capítulo. Cuando un término condicione una decisión, sigue el enlace y
revisa su contexto completo.
