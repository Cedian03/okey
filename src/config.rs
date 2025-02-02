use embassy_usb::{class::hid::Config as HidConfig, Config as UsbConfig};

use crate::usb::KEYBOARD_DESCRIPTOR;

pub struct Config<'a>
{
    pub usb_config: UsbConfig<'a>,
    pub hid_config: HidConfig<'a>,
}

impl<'a> Config<'a> {
    pub fn new(vid: u16, pid: u16) -> Self {
        Self {
            usb_config: UsbConfig::new(vid, pid),
            hid_config: HidConfig { 
                report_descriptor: KEYBOARD_DESCRIPTOR, 
                request_handler: None, 
                poll_ms: 4, 
                max_packet_size: 64
            }
        }
    }
}