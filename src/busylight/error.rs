use std::fmt::{Display, Formatter};
use hidapi::{HidError, HidResult};
use crate::Light;

#[derive(Debug)]
pub enum LightError {
    HidError(HidError),
    InvalidColor(String)
}

impl From<HidError> for LightError {
    fn from(value: HidError) -> Self {
        LightError::HidError(value)
    }
}


pub type Result<T> = std::result::Result<T, LightError>;