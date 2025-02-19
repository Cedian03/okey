//! Everything needed for a standard matrix keyboard.

pub use crate::{
    Keyboard,
    action::Action,
    usb::{Code, Config, State, UsbInterface},
    scan::Matrix,
};