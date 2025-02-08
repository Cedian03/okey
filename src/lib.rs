#![no_std]
#![no_main]

pub mod prelude;

pub mod action;
pub mod action_map;
pub mod config;
pub mod keycode;
pub mod report;
pub mod scan;
pub mod usb;


use embassy_futures::join::join3;
use embassy_usb::{class::hid::{HidReaderWriter, State}, Builder};
use embassy_usb_driver::Driver;

use action::Action;
use action_map::ActionMap;
use config::Config;
use scan::KeyScan;
use report::Report;
use usb::handlers::{OkeyDeviceHandler, OkeyRequestHandler};

pub struct Buffers<'a> {
    pub config_descriptor_buf: [u8; 256],
    pub bos_descriptor_buf: [u8; 256],
    pub msos_descriptor_buf: [u8; 256],
    pub control_buf: [u8; 64],
    pub request_handler: OkeyRequestHandler,
    pub device_handler: OkeyDeviceHandler,
    pub state: State<'a>,
}

impl<'a> Default for Buffers<'a> {
    fn default() -> Self {
        Self {
            config_descriptor_buf: [0; 256],
            bos_descriptor_buf: [0; 256],
            msos_descriptor_buf: [0; 256],
            control_buf: [0; 64],
            request_handler: OkeyRequestHandler,
            device_handler: OkeyDeviceHandler,
            state: State::new(),
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
    
        builder.handler(&mut bufs.device_handler);
    
        let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut bufs.state, config.hid_config);
    
        let (reader, mut writer) = hid.split();
    
        let mut usb = builder.build();
    
        let usb_fut = usb.run();
    
        let in_fut = async {
            let mut scan = [[false; W]; H];
            let mut last_scan = [[false; W]; H];
            
            loop {
                self.scanner.scan(&mut scan).await; 

                for y in 0..H {
                    for x in 0..W {
                        if scan[y][x] != last_scan[y][x] {
                            self.handle_key_event(x, y, scan[y][x]);
                        }
                    }
                }

                let _ = writer.write(self.report.as_slice()).await;

                last_scan = scan;
            }
        };
    
        let out_fut = reader.run(false, &mut bufs.request_handler);
    
        let _ = join3(usb_fut, in_fut, out_fut).await;
        
        panic!()
    }

    fn handle_key_event(&mut self, x: usize, y: usize, pressed: bool) {
        if pressed {
            self.handle_key_pressed(x, y)
        } else {
            self.handle_key_released(x, y)
        }
    }

    fn handle_key_pressed(&mut self, x: usize, y: usize) {
        if let Some(action) = self.action_map.get(x, y) {
            assert!(
                self.current_action[y][x].replace(action).is_none(), 
                "Key ({}, {}) pressed twice without being relesed inbetween", x, y
            );

            match action {
                Action::Key(code) => {
                    if let Some(mask) = code.modifier_mask() {
                        self.report.register_modifier(mask);
                    } else {
                        let _ = self.report.register_code(code);
                    }
                },
                Action::MomentaryLayer(layer) => {
                    self.action_map.set_layer(layer)
                }
                Action::ToggleLayer(layer) => {
                    self.action_map.toggle_layer(layer)
                }
            }
        }

        // TODO: assert that keys with no action are registered correctly
    }

    fn handle_key_released(&mut self, x: usize, y: usize) {
        if let Some(action) = self.pop(x, y) {
            match action {
                Action::Key(code) => {
                    if let Some(mask) = code.modifier_mask() {
                        self.report.unregister_modifier(mask);
                    } else {
                        let _ = self.report.unregister_code(code);
                    }
                },
                Action::MomentaryLayer(layer) => {
                    self.action_map.unset_layer(layer);
                }
                Action::ToggleLayer(_) => {}
            }
        }

        // TODO: assert that keys with no action are registered correctly
    }

    fn pop(&mut self, x: usize, y: usize) -> Option<Action> {
        self.current_action[y][x].take()
    }
}