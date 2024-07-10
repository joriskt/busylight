use hidapi::{DeviceInfo, HidDevice};
use crate::busylight::{Light, Color, LightError, Result};

pub struct LuxaforLight<'b> {
    device_info: &'b DeviceInfo,
    hid_device: HidDevice
}

impl<'b> Light<'b> for LuxaforLight<'b> {

    fn new(device_info: &'b DeviceInfo, hid_device: HidDevice) -> Self {
        LuxaforLight {
            device_info,
            hid_device,
        }
    }

    fn get_info(&self) -> &'b DeviceInfo {
        self.device_info
    }

    fn turn_off(&self) -> Result<()> {
        self.hid_device.write(&[0x00, 0x01, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
            .map(|num| ())
            .map_err(|e| { LightError::HidError(e) })
    }

    fn set_solid_color(&self, color: Color) -> Result<()> {
        self.hid_device.write(&[0x00, 0x01, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00])
            .map(|num| ())
            .map_err(|e| { LightError::HidError(e) })
    }
}