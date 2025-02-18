use embassy_usb::{class::hid::Config as HidConfig, Config as UsbConfig};

use crate::usb::report::KEYBOARD_DESCRIPTOR;

pub struct Config<'a> {
    vid: u16,
    pid: u16,
    manufacturer: Option<&'a str>,
    product: Option<&'a str>,
}

impl<'a> Config<'a> {
    pub const fn new() -> Self {
        Self {
            vid: 0x1209,
            pid: 0x0001,
            manufacturer: None,
            product: None,
        }
    }

    pub const fn split(self) -> (UsbConfig<'a>, HidConfig<'a>) {
        let mut usb = UsbConfig::new(self.vid, self.pid);
        usb.manufacturer = self.manufacturer;
        usb.product = self.product;

        let hid = HidConfig {
            report_descriptor: KEYBOARD_DESCRIPTOR,
            request_handler: None,
            poll_ms: 10,
            max_packet_size: usb.max_packet_size_0 as u16,
        };

        (usb, hid)
    }

    pub const fn pid(mut self, pid: u16) -> Self {
        self.pid = pid;
        self
    }

    pub const fn vid(mut self, vid: u16) -> Self {
        self.vid = vid;
        self
    }

    pub const fn manufacturer(mut self, manufacturer: &'static str) -> Self {
        self.manufacturer = Some(manufacturer);
        self
    }

    pub const fn product(mut self, product: &'static str) -> Self {
        self.product = Some(product);
        self
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self::new()
    }
}