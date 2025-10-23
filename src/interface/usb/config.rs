use embassy_usb::{Config as UsbConfig, class::hid::Config as HidConfig};

use super::report::REPORT_DESCRIPTOR;

#[derive(Clone, Copy, Debug)]
pub struct Config<'a> {
    /// Vendor ID. Default: 0x1209.
    vid: u16,
    /// Product ID. Default: 0x0001.
    pid: u16,
    /// Manufacturer name. Default: None.
    manufacturer: Option<&'a str>,
    /// Product name. Default: None.
    product: Option<&'a str>,
    /// Serial number. Default: None.
    serial_number: Option<&'a str>,
    /// Polling interval in ms. Default: 10.
    poll_interval: u8,
}

#[cfg(feature = "defmt")]
impl<'a> defmt::Format for Config<'a> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Config {{ vid: {:#04x}, pid: {:#04x}, manufacturer: {:?}, product: {:?}, serial_number: {:?}, poll_interval: {} }}",
            self.vid,
            self.pid,
            self.manufacturer,
            self.product,
            self.serial_number,
            self.poll_interval
        )
    }
}

impl<'a> Config<'a> {
    pub const fn new() -> Self {
        Self {
            vid: 0x1209,
            pid: 0x0001,
            manufacturer: None,
            product: None,
            serial_number: None,
            poll_interval: 10,
        }
    }

    pub(super) const fn split(self) -> (UsbConfig<'a>, HidConfig<'a>) {
        let mut usb = UsbConfig::new(self.vid, self.pid);
        usb.manufacturer = self.manufacturer;
        usb.product = self.product;
        usb.serial_number = self.serial_number;

        let hid = HidConfig {
            report_descriptor: REPORT_DESCRIPTOR,
            request_handler: None,
            poll_ms: self.poll_interval,
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

    pub const fn manufacturer(mut self, manufacturer: &'a str) -> Self {
        self.manufacturer = Some(manufacturer);
        self
    }

    pub const fn product(mut self, product: &'a str) -> Self {
        self.product = Some(product);
        self
    }

    pub const fn serial_number(mut self, serial_number: &'a str) -> Self {
        self.serial_number = Some(serial_number);
        self
    }

    pub const fn poll_interval(mut self, ms: u8) -> Self {
        self.poll_interval = ms;
        self
    }

    pub const fn poll_rate(mut self, hz: u16) -> Self {
        assert!(hz >= 4 && hz <= 1000);
        self.poll_interval = (1000 / hz) as u8;
        self
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self::new()
    }
}
