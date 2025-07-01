mod layered;

use crate::Action;

pub use layered::{LayeredMap, Opacity};

pub trait ActionMap<const W: usize, const H: usize> {
    fn get(&self, x: u8, y: u8) -> Option<Action>;
}
