use pi_hole_api::{
    errors, AuthenticatedPiHoleAPI, PiHoleAPIConfig, PiHoleAPIConfigWithKey,
    UnauthenticatedPiHoleAPI,
};

pub enum PiHoleConfigImplementation {
    Default(PiHoleAPIConfig),
    WithKey(PiHoleAPIConfigWithKey),
}

impl Into<PiHoleConfigImplementation> for PiHoleAPIConfig {
    fn into(self) -> PiHoleConfigImplementation {
        PiHoleConfigImplementation::Default(self)
    }
}

impl Into<PiHoleConfigImplementation> for PiHoleAPIConfigWithKey {
    fn into(self) -> PiHoleConfigImplementation {
        PiHoleConfigImplementation::WithKey(self)
    }
}

impl PiHoleConfigImplementation {
    pub fn new(host: String, api_key: Option<String>) -> Self {
        match api_key {
            Some(key) => PiHoleAPIConfigWithKey::new(host, key).into(),
            None => PiHoleAPIConfig::new(host).into(),
        }
    }

    pub fn get_unauthenticated_api(
        &self,
    ) -> Result<&dyn UnauthenticatedPiHoleAPI, errors::APIError> {
        Ok(match self {
            Self::Default(config) => config,
            Self::WithKey(config) => config,
        })
    }

    pub fn get_authenticated_api(&self) -> Result<&dyn AuthenticatedPiHoleAPI, errors::APIError> {
        match self {
            Self::Default(_) => Err(errors::APIError::MissingAPIKey),
            Self::WithKey(config) => Ok(config),
        }
    }
}
