use core::ops::Index;

use crate::Action;

use super::ActionMap;

pub struct LayeredMap<const W: usize, const H: usize, const D: usize> {
    map: [[[Opacity<Option<Action>>; W]; H]; D],
    active: u32,
}

impl<const W: usize, const H: usize, const D: usize> LayeredMap<W, H, D> {
    const _ASSERT: () = assert!(D >= 1 && D <= 32);

    const WIDTH: u8 = W as u8;
    const HEIGHT: u8 = H as u8;
    const DEPTH: u8 = D as u8;

    pub const fn new(map: [[[Opacity<Option<Action>>; W]; H]; D]) -> Self {
        Self::with_active(map, 1)
    }

    pub const fn with_active(map: [[[Opacity<Option<Action>>; W]; H]; D], active: u32) -> Self {
        Self { map, active }
    }

    pub fn is_active(&self, layer: u8) -> bool {
        self.active & (1 << layer) != 0
    }

    pub fn activate_layer(&mut self, layer: u8) {
        self.active |= 1 << layer;
    }

    pub fn deactivate_layer(&mut self, layer: u8) {
        self.active &= !(1 << layer);
    }

    pub fn toggle_layer(&mut self, layer: u8) {
        self.active ^= 1 << layer;
    }
}

impl<const W: usize, const H: usize, const D: usize> ActionMap<W, H> for LayeredMap<W, H, D> {
    fn get(&self, x: u8, y: u8) -> Option<Action> {
        (0..Self::DEPTH)
            .rev()
            .filter(|z| self.is_active(*z))
            .find_map(|z| self[[x, y, z]].into())
            .unwrap_or_default()
    }
}

impl<const W: usize, const H: usize, const D: usize> Index<[u8; 3]> for LayeredMap<W, H, D> {
    type Output = Opacity<Option<Action>>;

    fn index(&self, index: [u8; 3]) -> &Self::Output {
        &self.map[index[2] as usize][index[1] as usize][index[0] as usize]
    }
}

impl<const W: usize, const H: usize, const D: usize> From<[[[Opacity<Option<Action>>; W]; H]; D]> for LayeredMap<W, H, D> {
    fn from(map: [[[Opacity<Option<Action>>; W]; H]; D]) -> Self {
        Self::new(map)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Opacity<T> {
    Opaque(T),
    Transparent,
}

impl<T> Into<Option<T>> for Opacity<T> {
    fn into(self) -> Option<T> {
        match self {
            Self::Opaque(x) => Some(x),
            Self::Transparent => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{codes::*, interface::usb::Code};

    use super::*;

    #[rustfmt::skip] 
    const TEST_MAP: LayeredMap<2, 2, 2> = LayeredMap::with_active(
        [
            [
                [KC_0,    KC_1   ],
                [KC_NO,   KC_TRNS],
            ],
            [
                [KC_A,    KC_TRNS],
                [KC_TRNS, KC_TRNS],
            ],
        ],
        0b11
    );

    #[test]
    fn action_for_opaque_action() {
        assert_eq!(TEST_MAP.get(0, 0), Some(Action::Code(Code::KeyboardA)));
    }

    #[test]
    fn action_for_transparent_over_oaction() {
        assert_eq!(TEST_MAP.get(1, 0), Some(Action::Code(Code::Keyboard1)));
    }

    #[test]
    fn none_for_none() {
        assert_eq!(TEST_MAP.get(0, 1), None);
    }

    #[test]
    fn none_for_transparent() {
        assert_eq!(TEST_MAP.get(1, 1), None);
    }
}
