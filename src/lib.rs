#![no_std]

pub mod prelude;

pub mod action;
pub mod action_map;
pub mod codes;
pub mod event;
pub mod interface;
pub mod scan;

use embassy_futures::join;
use embassy_time::{Duration, Instant, Ticker};

use action::Action;
use action_map::ActionMap;
use event::Event;
use interface::{Handler, Interface};
use scan::Scan;

pub const SCAN_INTERVAL: Duration = Duration::from_millis(1); // ms

pub const TAP_TIMEOUT: Duration = Duration::from_millis(1000);

pub struct Keyboard<S, I, const W: usize, const H: usize, const D: usize> {
    scanner: S,
    action_map: ActionMap<W, H, D>,
    interface: I,
}

impl<S, I, const W: usize, const H: usize, const D: usize> Keyboard<S, I, W, H, D>
where
    S: Scan<W, H>,
    I: Interface,
{
    pub fn new<M: Into<ActionMap<W, H, D>>>(scanner: S, map: M, interface: I) -> Self {
        Self {
            scanner,
            action_map: map.into(),
            interface,
        }
    }

    pub async fn run(self) -> ! {
        let (board, fut) = self.morph();
        join::join(board.run(), fut).await;
        unreachable!()
    }

    fn morph(self) -> (RunningKeyboard<S, I::Handler, W, H, D>, impl Future) {
        let (handler, fut) = self.interface.start();

        (
            RunningKeyboard::new(self.scanner, self.action_map, handler),
            fut,
        )
    }
}

struct RunningKeyboard<S, T, const W: usize, const H: usize, const D: usize> {
    scanner: S,
    action_map: ActionMap<W, H, D>,
    handler: T,
    current_action: [[Option<(Action, Option<Instant>)>; W]; H], 
}

impl<S, T, const W: usize, const H: usize, const D: usize> RunningKeyboard<S, T,  W, H, D>
where 
    S: Scan<W, H>,
    T: Handler,
{
    fn new(scanner: S, action_map: ActionMap<W, H, D>, handler: T) -> Self {
        Self {
            scanner,
            action_map,
            handler,
            current_action: [[None; W]; H],
        }
    }

    async fn run(mut self) -> ! {
        let mut scan = &mut [[false; W]; H];
        let mut prev_scan = &mut [[false; W]; H];

        let mut ticker = Ticker::every(SCAN_INTERVAL);

        loop {
            self.scanner.scan(scan).await; 
            self.process_events(scan, prev_scan);
            self.handler.flush();
            
            core::mem::swap(&mut scan, &mut prev_scan);
            
            ticker.next().await;
        }
    }

    fn process_events(&mut self, scan: &[[bool; W]; H], prev_scan: &[[bool; W]; H]) {
        for y in 0..H {
            for x in 0..W {
                if let Some(event) = self.get_event(x, y, scan[y][x], prev_scan[y][x]) {
                    self.process_event(x, y, event);
                }
            }
        }
    }

    fn process_event(&mut self, x: usize, y: usize, event: Event) {
        match event {
            Event::Pressed => self.process_key_pressed(x, y),
            Event::Released => self.process_key_released(x, y),
            Event::Held => self.process_key_held(x, y),
        }
    }

    fn process_key_pressed(&mut self, x: usize, y: usize) {
        let action = self.action_map.get(x, y);
        assert!(self.set_action(x, y, action).is_none());
        self.process_action_pressed(action)
    }

    fn process_action_pressed(&mut self, action: Action) {
        match action {
            Action::Code(code) => self.handler.register_code(code),
            Action::MomentaryLayer(layer) => self.action_map.set_layer(layer),
            Action::ToggleLayer(layer) => self.action_map.toggle_layer(layer),
            _ => {}
        }
    }

    fn process_key_held(&mut self, x: usize, y: usize) {
        let (action, _) = self.get_action(x, y).unwrap();
        self.process_action_held(action);
    }

    fn process_action_held(&mut self, action: Action) {
        match action {
            Action::TapHold(_, code) => self.handler.register_code(code),
            _ => {}
        }
    }

    fn process_key_released(&mut self, x: usize, y: usize) {
        let (action, tapped) = self.unset_action(x, y).unwrap();
        self.process_action_released(action, tapped)
    }

    fn process_action_released(&mut self, action: Action, tapped: bool) {
        match action {
            Action::Code(code) => self.handler.unregister_code(code),
            Action::TapHold(code, _) if tapped => self.handler.temp_register_code(code),
            Action::TapHold(_, code) => self.handler.unregister_code(code),
            Action::MomentaryLayer(layer) => self.action_map.unset_layer(layer),
            _ => {}
        }
    }

    fn get_event(&mut self, x: usize, y: usize, scan: bool, prev_scan: bool) -> Option<Event> {
        match (scan, prev_scan) {
            (true, false) => Some(Event::Pressed),
            (false, true) => Some(Event::Released),
            (true, true) => {
                self.get_action_mut(x, y)
                    .unwrap()
                    .1
                    .take_if(|p| p.elapsed() >= TAP_TIMEOUT)
                    .map(|_| Event::Held)
            },
            _ => None
        }
    }

    fn get_action(&self, x: usize, y: usize) -> Option<(Action, Option<Instant>)> {
        self.current_action[y][x]
    }

    fn get_action_mut(&mut self, x: usize, y: usize) -> Option<&mut (Action, Option<Instant>)> {
        self.current_action[y][x].as_mut()
    }

    fn set_action(&mut self, x: usize, y: usize, action: Action) -> Option<(Action, Option<Instant>)> {
        self.current_action[y][x].replace((action, Some(Instant::now())))
    }

    fn unset_action(&mut self, x: usize, y: usize) -> Option<(Action, bool)> {
        self.current_action[y][x].take().map(|(a, b)| (a, b.is_some()))
    }
}
