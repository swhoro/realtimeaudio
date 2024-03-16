use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, Host, SupportedStreamConfig,
};

pub struct Config {
    pub record_device: Option<Device>,
    pub record_device_config: Option<SupportedStreamConfig>,

    pub play_device: Option<Device>,
    pub play_device_config: Option<SupportedStreamConfig>,
}

impl Config {
    pub fn new(host: Host) -> Config {
        let mut record_device_config = None;
        let record_device = match host.default_input_device() {
            Some(device) => {
                record_device_config = match device.default_input_config() {
                    Ok(config) => Some(config),
                    Err(_) => {
                        println!("cannot get record device config");
                        None
                    }
                };
                Some(device)
            }
            None => {
                println!("cannot get record device");
                None
            }
        };

        let mut play_device_config = None;
        let play_device = match host.default_output_device() {
            Some(device) => {
                play_device_config = match device.default_output_config() {
                    Ok(config) => Some(config),
                    Err(_) => {
                        println!("cannot get record device config");
                        None
                    }
                };
                Some(device)
            }
            None => {
                println!("cannot get record device");
                None
            }
        };

        Config {
            record_device,
            record_device_config,
            play_device,
            play_device_config,
        }
    }

    pub fn change_record_device(&mut self, device: Device) {
        self.record_device = Some(device);
        self.record_device_config = match device.default_input_config() {
            Ok(config) => Some(config),
            Err(_) => {
                println!("cannot get record device config");
                None
            }
        };
    }

    pub fn change_play_device(&mut self, device: Device) {
        self.play_device = Some(device);
        self.play_device_config = match device.default_output_config() {
            Ok(config) => Some(config),
            Err(_) => {
                println!("cannot get record device config");
                None
            }
        };
    }
}
