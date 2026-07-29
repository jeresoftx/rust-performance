# Especificación: contrato de medición reproducible

**Estado:** draft  
**Issue:** #2  
**Capítulo relacionado:** 01, evidencia y contrato de medición

## Concepto

Una medición de rendimiento es evidencia condicionada: compara una línea base
con una variante bajo entradas y entorno declarados. No es una propiedad
intrínseca del código ni una promesa de mejora para cualquier equipo o carga.

## Problema

Cambiar código sin una hipótesis medible favorece explicaciones retrospectivas:
una ejecución aislada puede variar por calentamiento, optimizaciones del
compilador, ruido del sistema operativo o una entrada poco representativa. El
modelo educativo necesita conservar el contexto mínimo para que otra persona
pueda interpretar y repetir el experimento.

## Invariantes

Un experimento válido debe declarar:

1. **Hipótesis falsable:** relación esperada entre variante y métrica, además
   de la condición que la refutaría.
2. **Línea base:** implementación o configuración contra la que se compara la
   variante; no se sustituye después de observar resultados.
3. **Métrica y unidad:** qué se observa, cómo se expresa y si valores menores o
   mayores son mejores.
4. **Entrada:** tamaño, distribución, semilla cuando aplique y relación con la
   carga que se quiere representar.
5. **Entorno:** versión de Rust, perfil de compilación, arquitectura y
   condiciones relevantes conocidas.
6. **Repetición:** más de una observación, con calentamiento separado de las
   muestras reportadas cuando aplique.
7. **Limitaciones:** factores no controlados, amenaza a validez y alcance de la
   conclusión.

Un resultado solo se interpreta cuando base y variante preservan la misma
corrección observable para la entrada definida.

## Alternativas

| Alternativa | Ventaja | Límite | Decisión |
|---|---|---|---|
| Cronometrar una ejecución ad hoc | Rápida para explorar | No separa ruido, entrada ni entorno | Insuficiente como evidencia. |
| Guardar solo un promedio | Fácil de comunicar | Oculta dispersión y no explica el procedimiento | Insuficiente. |
| Medir con contrato explícito | Hace auditables hipótesis y límites | Requiere más contexto | Adoptada. |
| Usar una herramienta externa como sustituto del contrato | Puede automatizar mediciones | No decide qué comparación es válida | Complementaria, no obligatoria. |

## Modelo propuesto

El capítulo siguiente representará un experimento mediante datos: identidad,
hipótesis, variante base, variante candidata, configuración de entrada,
entorno, métrica, muestras y amenazas a validez. El modelo rechazará campos
vacíos esenciales y separará el calentamiento de las muestras observadas.

No calcula una mejora a partir de una sola muestra ni infiere causalidad. Las
pruebas verifican que el contrato no permita comparaciones ambiguas; los
benchmarks posteriores aportan observaciones, no sustituyen esas pruebas.

## Criterio educativo

La implementación elegida será una estructura de datos pequeña de biblioteca
estándar. Mantiene el foco en los invariantes y evita introducir una herramienta
de benchmarking antes de que el estudiante sepa formular la pregunta correcta.

## Límites

- El contrato no controla frecuencia de CPU, carga del sistema operativo ni
  microarquitectura.
- Declarar el entorno reduce ambigüedad, pero no garantiza reproducibilidad
  idéntica entre equipos.
- Una diferencia medida describe únicamente la entrada, configuración y
  entorno declarados.
- El modelo no sustituye profiling ni análisis estadístico especializado.
