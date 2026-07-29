# Especificación: asignaciones, reutilización y propiedad

**Estado:** draft  
**Issue:** #17

## Concepto

Asignar memoria tiene costo, pero evitar una asignación no es automáticamente
una mejora. Reutilizar un buffer puede reducir trabajo del allocator y también
retener memoria, complicar ownership o aumentar el costo de limpiar datos.

## Problema

Construir temporalmente cadenas, vectores o buffers en una ruta caliente puede
amplificar asignaciones. Antes de reutilizar, se debe comprobar que el resultado
observable es idéntico, que la capacidad requerida está declarada y que la vida
del buffer no cruza límites de ownership incorrectos.

## Invariantes

1. La construcción nueva y la reutilizada producen los mismos bytes o valores.
2. Reutilizar no expone contenido anterior como resultado actual.
3. La capacidad es una decisión explícita, no una promesa de que nunca habrá
   crecimiento.
4. El benchmark separa generar entrada de construir salida.
5. El reporte incluye tamaño y distribución de la salida, porque cambian
   asignaciones y crecimiento.

## Alternativas

| Alternativa | Decisión | Límite |
|---|---|---|
| Asignar por claridad | Válida como línea base | Puede ser costosa en rutas repetidas. |
| Reutilizar todo buffer | Rechazada | Puede retener memoria o hacer ownership opaco. |
| Reutilizar cuando la vida y capacidad lo justifican | Adoptada | Requiere pruebas de contenido y límites. |

## Modelo educativo

#18 implementará construcción de bytes con buffer nuevo y con `clear` más
reutilización. Las pruebas verificarán que no quedan bytes previos y que ambas
variantes producen la misma salida. El modelo usa biblioteca estándar y no
intenta contar asignaciones del allocator.

## Límites

Capacidad, allocator, tamaño de objeto y plataforma afectan resultados. El
curso enseña a formular la hipótesis y comparar; no afirma que `Vec::with_capacity`
o `clear` gane para toda carga.
