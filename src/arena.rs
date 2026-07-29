//! Arena educativa segura basada en índices y generaciones.

/// Identificador válido únicamente dentro de una generación de arena.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArenaId {
    index: usize,
    generation: u64,
}

/// Colección de valores con invalidación explícita mediante `reset`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SafeArena<T> {
    values: Vec<T>,
    generation: u64,
}

impl<T> SafeArena<T> {
    /// Construye una arena sin capacidad reservada.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            values: Vec::new(),
            generation: 0,
        }
    }

    /// Construye una arena con capacidad inicial explícita.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            generation: 0,
        }
    }

    /// Inserta un valor y devuelve un identificador de la generación actual.
    pub fn insert(&mut self, value: T) -> ArenaId {
        let id = ArenaId {
            index: self.values.len(),
            generation: self.generation,
        };
        self.values.push(value);
        id
    }

    /// Obtiene un valor si su identificador pertenece a la generación actual.
    #[must_use]
    pub fn get(&self, id: ArenaId) -> Option<&T> {
        (id.generation == self.generation)
            .then(|| self.values.get(id.index))
            .flatten()
    }

    /// Elimina todos los valores e invalida los identificadores anteriores.
    pub fn reset(&mut self) {
        self.values.clear();
        self.generation = self.generation.wrapping_add(1);
    }
}

impl<T> Default for SafeArena<T> {
    fn default() -> Self {
        Self::new()
    }
}
