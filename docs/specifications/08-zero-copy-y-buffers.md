# Especificación: zero-copy, buffers y parsing

**Estado:** draft  
**Issue:** #24

## Concepto

Zero-copy describe preservar préstamos a datos existentes en vez de crear una
representación propietaria nueva. Es útil solo cuando el propietario vive lo
suficiente, la interfaz puede expresar el préstamo y el costo de la copia es
relevante para la carga.

## Problema

Un parser que convierte cada fragmento en `String` o `Vec<u8>` simplifica
ownership a costa de copiar. Un parser prestado evita copias y también acopla
la salida al buffer de entrada. El curso debe comparar las dos decisiones sin
presentar préstamo como optimización universal.

## Invariantes

1. Parser con copia y parser prestado reconocen el mismo formato.
2. Un campo prestado nunca sobrevive a su buffer de entrada.
3. Separadores, entradas vacías e inválidas tienen semántica explícita.
4. El benchmark separa crear la entrada de parsearla.
5. El reporte declara tamaño, codificación y duración del buffer.

## Alternativas

| Alternativa | Decisión | Límite |
|---|---|---|
| Copiar siempre | Línea base clara | Puede duplicar memoria y trabajo. |
| Prestar siempre | Rechazada | La salida puede requerir independencia. |
| Elegir según vida y frontera de API | Adoptada | Requiere expresar lifetimes y validar formato. |

## Modelo educativo

#25 implementará parsing de pares `clave=valor` separados por `;`, tanto como
slice prestado como copia propietaria. Las pruebas preservarán los mismos pares
y rechazarán segmentos sin separador. No se usará `unsafe` ni serialización
externa.

## Límites

Zero-copy no evita todas las copias: puede haber buffers de red, normalización o
fronteras de concurrencia. La decisión se mide en la carga declarada y se
evalúa junto con claridad de ownership.
