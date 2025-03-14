//! Everything needed for a standard matrix keyboard.

pub use crate::{
    action::Action,
    usb::{Code, Config, State, UsbInterface},
    scan::{debounce::Simple, Matrix, Scan},
    Keyboard,
};