use crate::keycode::KeyCode;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Key(KeyCode),
}