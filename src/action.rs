use crate::interface::usb::Code;

#[derive(Clone, Copy, Debug, Default)]
pub enum Action {
    #[default]
    NoAction,
    Code(Code),
    TapHold(Code, Code),
    MomentaryLayer(u8),
    ToggleLayer(u8),
}