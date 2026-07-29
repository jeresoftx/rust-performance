# Especificación: profiling y hot paths

**Estado:** draft  
**Issue:** #8

## Concepto

Un perfil atribuye muestras o contadores a partes de una ejecución. Sirve para
priorizar investigación: indica dónde se observó trabajo bajo una carga, no por
qué ocurre ni qué cambio lo reducirá.

## Problema

Optimizar la función que parece costosa en una lectura superficial puede mover
complejidad, romper corrección o ignorar el camino que domina la carga real. El
curso necesita separar tres preguntas: qué carga se perfiló, dónde se observó
tiempo y qué hipótesis posterior se debe medir.

## Invariantes

1. Un perfil declara carga, entorno, herramienta y modo de compilación.
2. Un hot path es una prioridad de investigación, no un diagnóstico causal.
3. Tiempo inclusivo y exclusivo no se intercambian: el primero incluye llamadas
   descendientes y el segundo intenta aislar el trabajo local.
4. Un contador solo se interpreta con su unidad y fuente.
5. Toda optimización propuesta vuelve al contrato de medición del capítulo 01.

## Alternativas

| Alternativa | Decisión | Límite |
|---|---|---|
| Leer código por intuición | Rechazada como evidencia | No representa la carga ejecutada. |
| Perfil como prueba de causa | Rechazada | Un perfil observa, no controla variables. |
| Perfil para formular hipótesis | Adoptada | Requiere benchmark posterior para comparar. |

## Modelo educativo

El modelo de #9 representará muestras atribuidas por ruta de llamada y
distinguirá tiempo inclusivo de exclusivo. Validará que los contadores sean no
negativos y que una ruta tenga identidad. No invocará un profiler real: el foco
es interpretar datos sin prometer que una herramienta concreta sea universal.

## Límites

La resolución de símbolos, muestreo, instrumentación y optimización del
compilador pueden alterar lo que un perfil muestra. Un resultado de perfil no
reemplaza pruebas de corrección ni mediciones comparativas.
