# Especificación: arena educativa segura

**Estado:** draft  
**Issue:** #21

## Concepto

Una arena agrupa valores con un ciclo de vida común. Una arena segura no entrega
referencias que sobrevivan a un reset ni depende de punteros crudos: conserva
los valores en un `Vec` y devuelve identificadores de índice verificables.

## Problema

Asignar muchos objetos pequeños puede ser relevante en una carga, pero diseñar
una arena con referencias estables suele requerir `unsafe` y contratos de vida
complejos. El curso necesita enseñar agrupación y reset sin ocultar esos
trade-offs ni violar la prohibición de `unsafe`.

## Invariantes

1. Un `ArenaId` es válido solo mientras apunta a la generación actual.
2. `reset` invalida todos los identificadores anteriores incrementando la
   generación.
3. `get` nunca produce una referencia para un índice o generación inválidos.
4. La capacidad se puede reservar, pero no garantiza ausencia de crecimiento.
5. La arena no devuelve referencias mutables simultáneas ni simula estabilidad
   de dirección.

## Alternativas

| Alternativa | Decisión | Límite |
|---|---|---|
| `Vec<T>` sin identificadores | Complementaria | No expresa invalidez por reset. |
| Arena de referencias con `unsafe` | Rechazada | Requiere invariantes que este curso no autoriza. |
| `Vec<T>` + índice y generación | Adoptada | Tiene chequeo y no ofrece direcciones estables. |

## Modelo educativo

#22 implementará `SafeArena<T>`, `insert`, `get` y `reset`. Las pruebas
demostrarán acceso válido, capacidad inicial y rechazo de IDs anteriores a un
reset. El modelo no cuenta llamadas al allocator ni reemplaza una arena de
producción especializada.

## Límites

El costo de validación y el crecimiento del vector forman parte de la
representación. Esta arena enseña ciclo de vida compartido de forma segura; no
promete rendimiento ni sirve como sustituto de un allocator general.
