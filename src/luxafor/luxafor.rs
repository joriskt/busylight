use hidapi::{DeviceInfo, HidDevice};
use crate::busylight::{Light, Color, LightError, Result};
use crate::LightType;

#[derive(Debug)]
pub struct LuxaforLight<'b> {
    light_type: LightType,
    device_info: &'b DeviceInfo,
    hid_device: HidDevice,
}

impl<'b> LuxaforLight<'b> {
    pub fn new(light_type: LightType,
               device_info: &'b DeviceInfo,
               hid_device: HidDevice) -> Self {
        LuxaforLight {
            light_type,
            device_info,
            hid_device,
        }
    }
}

impl<'b> Light<'b> for LuxaforLight<'b> {
    fn get_info(&self) -> &'b DeviceInfo {
        self.device_info
    }

    fn get_type(&self) -> &LightType {
        &self.light_type
    }

    fn turn_off(&self) -> Result<()> {
        self.hid_device.write(&[0x00, 0x01, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
            .map(|num| ())
            .map_err(|e| { LightError::HidError(e) })
    }

    fn set_solid_color(&self, color: &Color) -> Result<()> {
        let rgb = color.to_rgb();
        self.hid_device.write(&[0x00, 0x01, 0xFF, rgb.0, rgb.1, rgb.2, 0x00, 0x00, 0x00])
            .map(|num| ())
            .map_err(|e| { LightError::HidError(e) })
    }

    fn fade_to_color(&self, color: &Color, time: u8) -> Result<()> {
        let rgb = color.to_rgb();
        self.hid_device.write(&[0x00, 0x02, 0xFF, rgb.0, rgb.1, rgb.2, time, 0x00, 0x00])
            .map(|num| ())
            .map_err(|e| { LightError::HidError(e) })
    }
}