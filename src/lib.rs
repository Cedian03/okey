#![no_std]

pub mod prelude;

pub mod action;
pub mod action_map;
pub mod codes;
pub mod scan;
pub mod usb;


use action::Action;
use action_map::ActionMap;
use scan::Scan;
use usb::Report;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    Pressed,
    Released,
}

impl Event {
    pub fn new(pressed: bool) -> Self {
        if pressed {
            Self::Pressed
        } else {
            Self::Released
        }
    }
}

pub struct Keyboard<S, const W: usize, const H: usize, const D: usize> {
    scanner: S,
    action_map: ActionMap<W, H, D>,
    current_action: [[Option<Action>; W]; H], 
}

impl<S, const W: usize, const H: usize, const D: usize> Keyboard<S, W, H, D>
where
    S: Scan<W, H>,
{
    pub fn new(scanner: S, map: [[[Option<Action>; W]; H]; D]) -> Self {
        Self {
            scanner,
            action_map: ActionMap::new(map),
            current_action: [[None; W]; H],
        }
    }

    fn handle_key_event(&mut self, x: usize, y: usize, event: Event, report: &mut Report) {
        match event {
            Event::Pressed => self.handle_key_pressed(x, y, report),
            Event::Released => self.handle_key_released(x, y, report),
        }
    }

    // TODO: Move usb specific report usage.
    fn handle_key_pressed(&mut self, x: usize, y: usize, report: &mut Report) {
        let action = self.action_map.get(x, y);

        assert!(
            self.press(x, y, action).is_none(), 
            "Key ({}, {}) pressed twice without being released inbetween", x, y
        );

        match action {
            Action::NoAction => {}
            Action::Code(code) => {
                let _ = report.add(code);
            }
            Action::MomentaryLayer(layer) => {
                self.action_map.set_layer(layer)
            }
            Action::ToggleLayer(layer) => {
                self.action_map.toggle_layer(layer)
            }
        }
    }

    // TODO: Move usb specific report usage.
    fn handle_key_released(&mut self, x: usize, y: usize, report: &mut Report) {
        if let Some(action) = self.release(x, y) {
            match action {
                Action::NoAction => {}
                Action::Code(code) => {
                    let _ = report.remove(code);
                }
                Action::MomentaryLayer(layer) => {
                    self.action_map.unset_layer(layer);
                }
                Action::ToggleLayer(_) => {}
            }
        } else {
            panic!("Key ({}, {}) released without being pressed", x, y)
        }
    }

    fn press(&mut self, x: usize, y: usize, action: Action) -> Option<Action> {
        self.current_action[y][x].replace(action)
    }

    fn release(&mut self, x: usize, y: usize) -> Option<Action> {
        self.current_action[y][x].take()
    }
}