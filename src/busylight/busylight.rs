use hidapi::{DeviceInfo, HidApi, HidDevice};

use crate::{Light, LightType, LightError, LuxaforLight};
use crate::LightError::HidError;

pub struct Busylight {
    hid_api: HidApi,
}

impl<'b> Busylight {

    pub fn new() -> crate::Result<Self> where Self: Sized {
        match HidApi::new() {
            Ok(hid_api) => Ok(Self { hid_api }),
            Err(err) => Err(HidError(err))
        }
    }

    fn find_driver(&self, vendor_id: u16, product_id: u16) -> Option<LightType> {
        match (vendor_id, product_id) {
            (0x04D8, 0xF372) => Some(LightType::Luxafor),
            _ => None
        }
    }

    fn init_light(&'b self,
                  device_info: &'b DeviceInfo,
                  hid_device: crate::Result<HidDevice>,
                  driver: LightType) -> crate::Result<Box<dyn Light + 'b>> {
        match driver {
            LightType::Luxafor => Ok(Box::new(LuxaforLight::new(driver, device_info, hid_device?)))
        }
    }

    pub fn list_lights(&'b self) -> crate::Result<Vec<Box<dyn Light + 'b>>> {
        let list = self.hid_api.device_list()
            .map(|device_info| (device_info, self.find_driver(device_info.vendor_id(), device_info.product_id())))
            .filter(|tuple| tuple.1.is_some());

        let mut lights: Vec<Box<dyn Light>> = vec![];
        for (device_info, driver) in list {
            let device: crate::Result<HidDevice> = device_info.open_device(&self.hid_api)
                .map_err(|err| HidError(err));
            let light = self.init_light(device_info, device, driver.unwrap())?;
            lights.push(light);
        }

        Ok(lights)
    }
}

