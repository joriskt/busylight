use std::fmt::{Debug, Display, Formatter};

use hidapi::DeviceInfo;

use crate::busylight::{Color, Result};

#[derive(Debug)]
pub enum LightType {
    Luxafor,
}

impl Display for LightType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Light<'b> : Debug {

    fn get_info(&self) -> &'b DeviceInfo;
    fn get_type(&self) -> &LightType;

    fn turn_off(&self) -> Result<()>;
    fn set_solid_color(&self, color: &Color) -> Result<()>;
    fn fade_to_color(&self, color: &Color, time: u8) -> Result<()>;

}