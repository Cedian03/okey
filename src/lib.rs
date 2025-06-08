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
use action_map::{ActionMap, LayeredMap};
use event::Event;
use interface::{Handler, Interface};
use scan::Scan;

pub const SCAN_INTERVAL: Duration = Duration::from_millis(1);

pub const TAP_TIMEOUT: Duration = Duration::from_millis(1000);

pub struct Keyboard<S, M, I, const W: usize, const H: usize> {
    scanner: S,
    mapper: M,
    interface: I,
}

impl<S, I, const W: usize, const H: usize, const D: usize> Keyboard<S, LayeredMap<W, H, D>, I, W, H>
where
    S: Scan<W, H>,
    I: Interface,
{
    pub fn new<M: Into<LayeredMap<W, H, D>>>(scanner: S, mapper: M, interface: I) -> Self {
        Self {
            scanner,
            mapper: mapper.into(),
            interface,
        }
    }

    pub async fn run(self) -> ! {
        let (board, fut) = self.morph();
        join::join(board.run(), fut).await;
        unreachable!()
    }

    fn morph(
        self,
    ) -> (
        RunningKeyboard<S, LayeredMap<W, H, D>, I::Handler, W, H>,
        impl Future,
    ) {
        let (handler, fut) = self.interface.start();

        (
            RunningKeyboard::new(self.scanner, self.mapper, handler),
            fut,
        )
    }
}

struct RunningKeyboard<S, M, T, const W: usize, const H: usize> {
    scanner: S,
    mapper: M,
    handler: T,
    pressed: [[Option<Pressed>; W]; H],
}

impl<S, T, const W: usize, const H: usize, const D: usize>
    RunningKeyboard<S, LayeredMap<W, H, D>, T, W, H>
where
    S: Scan<W, H>,
    T: Handler,
{
    fn new(scanner: S, mapper: LayeredMap<W, H, D>, handler: T) -> Self {
        Self {
            scanner,
            mapper,
            handler,
            pressed: [[None; W]; H],
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
        if let Some(action) = self.mapper.get(x, y) {
            assert!(self.register_pressed(x, y, action).is_none());
            self.process_action_pressed(action)
        }
    }

    fn process_action_pressed(&mut self, action: Action) {
        match action {
            Action::Code(code) => self.handler.register(code),
            Action::MomentaryLayer(layer) => self.mapper.set_layer(layer),
            Action::ToggleLayer(layer) => self.mapper.toggle_layer(layer),
            _ => {}
        }
    }

    fn process_key_held(&mut self, x: usize, y: usize) {
        let action = {
            let pressed = self.get_pressed_mut(x, y).as_mut().unwrap();
            pressed.is_yet_to_be_held = false;
            pressed.action
        };

        self.process_action_held(action);
    }

    fn process_action_held(&mut self, action: Action) {
        match action {
            Action::TapHold(_, code) => self.handler.register(code),
            _ => {}
        }
    }

    fn process_key_released(&mut self, x: usize, y: usize) {
        let (action, was_held) = self.register_released(x, y).unwrap();
        self.process_action_released(action, !was_held)
    }

    fn process_action_released(&mut self, action: Action, was_tapped: bool) {
        match action {
            Action::Code(code) => self.handler.unregister(code),
            Action::TapHold(code, _) if was_tapped => self.handler.temp_register(code),
            Action::TapHold(_, code) => self.handler.unregister(code),
            Action::MomentaryLayer(layer) => self.mapper.unset_layer(layer),
            _ => {}
        }
    }

    fn get_event(&mut self, x: usize, y: usize, scan: bool, prev_scan: bool) -> Option<Event> {
        match (scan, prev_scan) {
            (true, false) => Some(Event::Pressed),
            (false, true) => Some(Event::Released),
            (true, true) => self
                .get_pressed(x, y)
                .unwrap()
                .is_ready_to_be_held()
                .then_some(Event::Held),
            _ => None,
        }
    }

    fn register_pressed(&mut self, x: usize, y: usize, action: Action) -> Option<Pressed> {
        self.get_pressed_mut(x, y).replace(Pressed::now(action))
    }

    fn register_released(&mut self, x: usize, y: usize) -> Option<(Action, bool)> {
        self.get_pressed_mut(x, y)
            .take()
            .map(|x| (x.action, x.is_yet_to_be_held))
    }

    fn get_pressed(&self, x: usize, y: usize) -> Option<Pressed> {
        self.pressed[y][x]
    }

    fn get_pressed_mut(&mut self, x: usize, y: usize) -> &mut Option<Pressed> {
        &mut self.pressed[y][x]
    }
}

#[derive(Clone, Copy, Debug)]
struct Pressed {
    action: Action,
    since: Instant,
    is_yet_to_be_held: bool,
}

impl Pressed {
    fn now(action: Action) -> Self {
        Self {
            action,
            since: Instant::now(),
            is_yet_to_be_held: true,
        }
    }

    fn is_ready_to_be_held(&self) -> bool {
        self.is_yet_to_be_held && self.since.elapsed() >= TAP_TIMEOUT
    }
}
