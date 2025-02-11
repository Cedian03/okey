use embassy_usb::class::hid::State;

use super::{OkeyDeviceHandler, OkeyRequestHandler};

pub struct Buffers<'a> {
    pub config_descriptor_buf: [u8; 256],
    pub bos_descriptor_buf: [u8; 256],
    pub msos_descriptor_buf: [u8; 256],
    pub control_buf: [u8; 64],
    pub request_handler: OkeyRequestHandler,
    pub device_handler: OkeyDeviceHandler,
    pub state: State<'a>,
}

impl<'a> Default for Buffers<'a> {
    fn default() -> Self {
        Self {
            config_descriptor_buf: [0; 256],
            bos_descriptor_buf: [0; 256],
            msos_descriptor_buf: [0; 256],
            control_buf: [0; 64],
            request_handler: OkeyRequestHandler,
            device_handler: OkeyDeviceHandler,
            state: State::new(),
        }
    }
}
