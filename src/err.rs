use std::{error::Error, fmt::{Debug, Display}};

use cpal::DefaultStreamConfigError;

#[derive(Debug)]
pub enum RError {
    DeviceError(String),
    CofigError(String)
}

impl Display for RError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<DefaultStreamConfigError> for RError {
    fn from(value: DefaultStreamConfigError) -> Self {
        RError::DeviceError("DeviceError:".to_string() + &value.to_string())
    }
}

impl Error for RError {}
