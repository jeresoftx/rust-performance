//! Modelos de construcción nueva y reutilización explícita de buffers.

/// Construye un mensaje en un buffer nuevo para establecer una línea base.
#[must_use]
pub fn build_message_fresh(payload: &str) -> Vec<u8> {
    let mut output = Vec::with_capacity(4 + payload.len());
    output.extend_from_slice(b"msg:");
    output.extend_from_slice(payload.as_bytes());
    output
}

/// Reutiliza el buffer proporcionado y devuelve la vista de su salida actual.
pub fn build_message_reused<'a>(buffer: &'a mut Vec<u8>, payload: &str) -> &'a [u8] {
    buffer.clear();
    buffer.extend_from_slice(b"msg:");
    buffer.extend_from_slice(payload.as_bytes());
    buffer.as_slice()
}
