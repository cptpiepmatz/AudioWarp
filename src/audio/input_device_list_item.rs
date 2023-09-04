use std::fmt::{Debug, Display, Formatter};

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{default_host, Device};

pub struct InputDeviceListItem {
    pub device: Device,
    pub name: String,
    pub is_default: bool
}

impl Debug for InputDeviceListItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        struct Device;

        impl Debug for Device {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "<Device>")
            }
        }

        f.debug_struct("InputDeviceListItem")
            .field("device", &Device)
            .field("name", &self.name)
            .field("is_default", &self.is_default)
            .finish()
    }
}

impl Display for InputDeviceListItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

pub fn list_input_devices() -> anyhow::Result<Vec<InputDeviceListItem>> {
    let host = default_host();
    let input_devices = host.input_devices()?;
    let Some(default_input_device) = host.default_input_device()
    else {
        return Ok(Vec::new());
    };
    let default_input_device_name = default_input_device.name()?;

    let size_hint = input_devices.size_hint();
    let mut devices = Vec::with_capacity(size_hint.1.unwrap_or(size_hint.0));
    for input_device in input_devices {
        let name = input_device.name()?;
        let is_default = name == default_input_device_name;
        devices.push(InputDeviceListItem {
            device: input_device,
            name,
            is_default
        });
    }

    Ok(devices)
}
