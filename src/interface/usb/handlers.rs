use embassy_usb::{class::hid::RequestHandler, Handler};

pub struct OkeyRequestHandler;

impl RequestHandler for OkeyRequestHandler {
    // TODO
}

pub struct OkeyDeviceHandler;

impl Handler for OkeyDeviceHandler {
    // TODO
}