use crate::action::Action;

pub struct ActionMap<const W: usize, const H: usize, const D: usize> {
    map: [[[Option<Action>; W]; H]; D],
    active_layers: u32,
}

impl<const W: usize, const H: usize, const D: usize> ActionMap<W, H, D> {
    pub const fn new(map: [[[Option<Action>; W]; H]; D]) -> Self {
        Self { map, active_layers: 1 }
    }

    pub fn get(&self, x: usize, y: usize) -> Action {
        (0..D)
            .rev()
            .filter(|z| self.is_active(*z as u8))
            .find_map(|z| self.index(x, y, z))
            .unwrap_or_default()
    }

    fn index(&self, x: usize, y: usize, z: usize) -> Option<Action> {
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

impl<const W: usize, const H: usize, const D: usize> From<[[[Option<Action>; W]; H]; D]> for ActionMap<W, H, D> {
    fn from(map: [[[Option<Action>; W]; H]; D]) -> Self {
        Self::new(map)
    }
}