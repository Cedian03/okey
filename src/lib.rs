#![no_std]

pub mod prelude;

pub mod action;
pub mod action_map;
pub mod codes;
pub mod event;
pub mod scan;
pub mod usb;

use action::Action;
use action_map::ActionMap;
use event::Event;
use scan::Scan;

pub struct Keyboard<S, const W: usize, const H: usize, const D: usize> {
    scanner: S,
    action_map: ActionMap<W, H, D>,
    current_action: [[Option<Action>; W]; H], 
}

impl<S, const W: usize, const H: usize, const D: usize> Keyboard<S, W, H, D>
where
    S: Scan<W, H>,
{
    pub fn new(scanner: S, map: impl Into<ActionMap<W, H, D>>) -> Self {
        Self {
            scanner,
            action_map: map.into(),
            current_action: [[None; W]; H],
        }
    }

    fn process_event(&mut self, x: usize, y: usize, event: Event) -> Action {
        match event {
            Event::Pressed => self.process_pressed(x, y),
            Event::Released => self.process_released(x, y),
        }
    }

    fn process_pressed(&mut self, x: usize, y: usize) -> Action {
        let action = self.action_map.get(x, y);

        assert!(self.set_action(x, y, action).is_none());

        match action {
            Action::MomentaryLayer(layer) => self.action_map.set_layer(layer),
            Action::ToggleLayer(layer) => self.action_map.toggle_layer(layer),
            _ => {}
        }

        action
    }

    fn process_released(&mut self, x: usize, y: usize) -> Action {
        let action = self.unset_action(x, y).unwrap();

        match action {
            Action::MomentaryLayer(layer) => self.action_map.unset_layer(layer),
            _ => {}
        }

        action
    }

    fn set_action(&mut self, x: usize, y: usize, action: Action) -> Option<Action> {
        self.current_action[y][x].replace(action)
    }

    fn unset_action(&mut self, x: usize, y: usize) -> Option<Action> {
        self.current_action[y][x].take()
    }
}
