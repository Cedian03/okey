//! Everything needed for a standard matrix keyboard.

pub use crate::{
    action::Action,
    usb::{Code, Config, State, UsbInterface},
    scan::{Matrix, Scan},
    Keyboard,
};