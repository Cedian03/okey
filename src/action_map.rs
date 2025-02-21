use crate::action::Action;

pub struct ActionMap<const W: usize, const H: usize, const D: usize> {
    layers: [[[Option<Action>; W]; H]; D],
    active_layers: u32,
}

impl<const W: usize, const H: usize, const D: usize> ActionMap<W, H, D> {
    pub fn new(map: [[[Option<Action>; W]; H]; D]) -> Self {
        Self { layers: map, active_layers: 1 }
    }

    pub fn get(&self, x: usize, y: usize) -> Action {
        (0..D)
            .rev()
            .filter(|z| self.is_active(*z as u8))
            .find_map(|z| self.index(x, y, z))
            .unwrap_or_default()
    }

    fn index(&self, x: usize, y: usize, z: usize) -> Option<Action> {
        self.layers[z][y][x]
    }

    pub fn is_active(&self, layer: u8) -> bool {
        self.active_layers & 1 << layer != 0
    }

    pub fn set_layer(&mut self, layer: u8) {
        self.active_layers |= 1 << layer
    }

    pub fn unset_layer(&mut self, layer: u8) {
        self.active_layers &= !(1 << layer)
    }

    pub fn toggle_layer(&mut self, layer: u8) {
        self.active_layers ^= 1 << layer
    }
}