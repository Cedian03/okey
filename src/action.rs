use crate::interface::usb::Code;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Code(Code),
    TapHold(Code, Code),
    MomentaryLayer(u8),
    ToggleLayer(u8),
}
