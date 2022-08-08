use pi_hole_api::{
    errors, AuthenticatedPiHoleAPI, PiHoleAPIConfig, PiHoleAPIConfigWithKey,
    UnauthenticatedPiHoleAPI,
};

pub enum PiHoleConfigImplementation {
    Default(PiHoleAPIConfig),
    WithKey(PiHoleAPIConfigWithKey),
}

impl PiHoleConfigImplementation {
    pub fn new(host: String, api_key: Option<String>) -> Self {
        match api_key {
            Some(key) => {
                PiHoleConfigImplementation::WithKey(PiHoleAPIConfigWithKey::new(host, key))
            }
            None => PiHoleConfigImplementation::Default(PiHoleAPIConfig::new(host)),
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
