//! Parsing equivalente con datos prestados o propietarios.

/// Error al interpretar un segmento de pares clave-valor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// El segmento no contiene el separador `=`.
    MissingSeparator,
}

/// Interpreta pares `clave=valor` sin copiar sus fragmentos.
pub fn parse_borrowed(input: &str) -> Result<Vec<(&str, &str)>, ParseError> {
    input
        .split(';')
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.split_once('=').ok_or(ParseError::MissingSeparator))
        .collect()
}

/// Interpreta el mismo formato y crea una representación propietaria.
pub fn parse_owned(input: &str) -> Result<Vec<(String, String)>, ParseError> {
    parse_borrowed(input).map(|pairs| {
        pairs
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value.to_owned()))
            .collect()
    })
}
