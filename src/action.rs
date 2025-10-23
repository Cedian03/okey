use crate::interface::usb::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Action {
    Code(KeyCode),
    // ModTap(Modifier, KeyCode),
    // LayerTap(Layer, KeyCode),
    TapHold { tap: KeyCode, hold: KeyCode },
    MomentaryLayer(u8),
    ToggleLayer(u8),
}
