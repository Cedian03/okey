use crate::keycode::KeyCode;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Key(KeyCode),
    MomentaryLayer(u8),
    ToggleLayer(u8),
}