//! Modelo educativo para interpretar atribución de trabajo en un perfil.

/// Una observación atribuida a una ruta de llamada.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileSample {
    path: String,
    inclusive_units: u64,
    exclusive_units: u64,
}

impl ProfileSample {
    /// Crea una muestra con trabajo inclusivo y exclusivo en unidades declaradas.
    #[must_use]
    pub fn new(path: impl Into<String>, inclusive_units: u64, exclusive_units: u64) -> Self {
        Self {
            path: path.into(),
            inclusive_units,
            exclusive_units,
        }
    }
}

/// Error al construir una interpretación de perfil.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProfileError {
    /// La ruta de llamada debe identificar el trabajo atribuido.
    MissingPath,
    /// El trabajo exclusivo no puede exceder el trabajo inclusivo.
    InvalidAttribution,
}

/// Perfil validado que ayuda a formular una hipótesis posterior.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Profile {
    samples: Vec<ProfileSample>,
}

impl Profile {
    /// Valida una serie de muestras atribuidas.
    pub fn new(samples: Vec<ProfileSample>) -> Result<Self, ProfileError> {
        for sample in &samples {
            if sample.path.trim().is_empty() {
                return Err(ProfileError::MissingPath);
            }
            if sample.exclusive_units > sample.inclusive_units {
                return Err(ProfileError::InvalidAttribution);
            }
        }
        Ok(Self { samples })
    }

    /// Devuelve la ruta con mayor trabajo inclusivo, si el perfil no está vacío.
    #[must_use]
    pub fn hottest_path(&self) -> &str {
        self.samples
            .iter()
            .max_by_key(|sample| sample.inclusive_units)
            .map_or("", |sample| sample.path.as_str())
    }

    /// Devuelve el trabajo inclusivo de una ruta.
    #[must_use]
    pub fn inclusive_units(&self, path: &str) -> Option<u64> {
        self.samples
            .iter()
            .find(|sample| sample.path == path)
            .map(|sample| sample.inclusive_units)
    }

    /// Devuelve el trabajo exclusivo de una ruta.
    #[must_use]
    pub fn exclusive_units(&self, path: &str) -> Option<u64> {
        self.samples
            .iter()
            .find(|sample| sample.path == path)
            .map(|sample| sample.exclusive_units)
    }
}
