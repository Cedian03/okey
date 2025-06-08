use crate::action::Action;

pub trait ActionMap<const W: usize, const H: usize> {
    fn get(&self, x: usize, y: usize) -> Option<Action>;
}

pub struct LayeredMap<const W: usize, const H: usize, const D: usize> {
    map: [[[Opacity<Option<Action>>; W]; H]; D],
    active_layers: u32,
}

impl<const W: usize, const H: usize, const D: usize> LayeredMap<W, H, D> {
    pub const fn new(map: [[[Opacity<Option<Action>>; W]; H]; D]) -> Self {
        Self {
            map,
            active_layers: 1,
        }
    }

    fn index(&self, x: usize, y: usize, z: usize) -> Opacity<Option<Action>> {
        self.map[z][y][x]
    }

    pub const fn active_layers(&self) -> u32 {
        self.active_layers
    }

    pub const fn set_active_layers(&mut self, active_layers: u32) -> u32 {
        core::mem::replace(&mut self.active_layers, active_layers)
    }

    pub const fn set_layer(&mut self, layer: u8) {
        self.active_layers |= 1 << layer
    }

    pub const fn unset_layer(&mut self, layer: u8) {
        self.active_layers &= !(1 << layer)
    }

    pub const fn toggle_layer(&mut self, layer: u8) {
        self.active_layers ^= 1 << layer
    }

    pub fn is_active(&self, layer: u8) -> bool {
        self.active_layers & 1 << layer != 0
    }
}

impl<const W: usize, const H: usize, const D: usize> ActionMap<W, H> for LayeredMap<W, H, D> {
    fn get(&self, x: usize, y: usize) -> Option<Action> {
        (0..D)
            .rev()
            .filter(|z| self.is_active(*z as u8))
            .find_map(|z| self.index(x, y, z).into())
            .unwrap_or_default()
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
