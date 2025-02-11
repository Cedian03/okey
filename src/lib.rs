#![no_std]

pub mod prelude;

pub mod action;
pub mod action_map;
pub mod codes;
pub mod scan;
pub mod usb;

use embassy_futures::join::join;
use embassy_usb::{class::hid::HidReaderWriter, driver::Driver, Builder};

use action::Action;
use action_map::ActionMap;
use scan::KeyScan;
use usb::{Buffers, Config, Report};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    Pressed,
    Released,
}

impl From<bool> for Event {
    fn from(value: bool) -> Self {
        if value {
            Self::Pressed
        } else {
            Self::Released
        }
    }
}

pub struct Keyboard<S, const W: usize, const H: usize, const D: usize>
{
    scanner: S,

    action_map: ActionMap<W, H, D>,
    current_action: [[Option<Action>; W]; H], 

    report: Report,
}

impl<'d, S, const W: usize, const H: usize, const D: usize> Keyboard<S, W, H, D>
where
    S: KeyScan<W, H> 
{
    pub fn new(scanner: S, map: [[[Option<Action>; W]; H]; D]) -> Self {
        Self {
            scanner,
            action_map: ActionMap::new(map),
            current_action: [[None; W]; H],
            report: Report::default(),
        }
    }

    pub async fn run<T: Driver<'d>>(
        mut self, 
        config: Config<'d>, 
        driver: T, 
        bufs: &'d mut Buffers<'d>
    ) -> ! {
        let mut builder = Builder::new(
            driver,
            config.usb_config,
            &mut bufs.config_descriptor_buf,
            &mut bufs.bos_descriptor_buf,
            &mut bufs.msos_descriptor_buf,
            &mut bufs.control_buf,
        );
    
        let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut bufs.state, config.hid_config);
        let (_reader, mut writer) = hid.split();
    
        let mut usb = builder.build();
        let usb_fut = usb.run();
    
        let key_fut = async {
            let mut scan = &mut [[false; W]; H];
            let mut last_scan = &mut [[false; W]; H];
            
            loop {
                self.scanner.scan(scan).await; 

                for y in 0..H {
                    for x in 0..W {
                        if scan[y][x] != last_scan[y][x] {
                            self.handle_key_event(x, y, scan[y][x].into());
                        }
                    }
                }

                let _ = writer.write(self.report.as_slice()).await;

                core::mem::swap(&mut scan, &mut last_scan);
            }
        };
    
        join(usb_fut, key_fut).await;
        
        unreachable!()
    }

    fn handle_key_event(&mut self, x: usize, y: usize, event: Event) {
        match event {
            Event::Pressed => self.handle_key_pressed(x, y),
            Event::Released => self.handle_key_released(x, y),
        }
    }

    fn handle_key_pressed(&mut self, x: usize, y: usize) {
        let action = self.action_map.get(x, y);

        assert!(
            self.press(x, y, action).is_none(), 
            "Key ({}, {}) pressed twice without being released inbetween", x, y
        );

        match action {
            Action::NoAction => {}
            Action::Key(code) => {
                let _ = self.report.register_code(code);
            }
            Action::Modifier(modifier) => {
                self.report.register_modifier(modifier)
            }
            Action::MomentaryLayer(layer) => {
                self.action_map.set_layer(layer)
            }
            Action::ToggleLayer(layer) => {
                self.action_map.toggle_layer(layer)
            }
        }
    }

    fn handle_key_released(&mut self, x: usize, y: usize) {
        if let Some(action) = self.release(x, y) {
            match action {
                Action::NoAction => {}
                Action::Key(code) => {
                    let _ = self.report.unregister_code(code);
                }
                Action::Modifier(modifier) => {
                    self.report.unregister_modifier(modifier);
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