use embassy_usb::{Handler, class::hid::RequestHandler};

pub struct OkeyRequestHandler;

impl RequestHandler for OkeyRequestHandler {
    // TODO
}

pub struct OkeyDeviceHandler;

impl Handler for OkeyDeviceHandler {
    // TODO
}
