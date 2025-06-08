//! Everything needed for a standard matrix keyboard.

pub use crate::{
    Keyboard,
    action::Action,
    interface::{
        Handler, Interface,
        usb::{Code, Config, State, UsbInterface},
    },
    scan::{Col2Row, Row2Col, Scan},
};
