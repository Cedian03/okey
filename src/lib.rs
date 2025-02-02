#![no_std]
#![no_main]

use embassy_futures::join::join3;
use embassy_usb::{class::hid::{HidReaderWriter, State}, Builder};
use embassy_usb_driver::Driver;

pub mod usb;
pub mod keycode;
pub mod config;
pub mod scan;
pub mod report;

use config::Config;
use keycode::KeyCode;
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

pub struct Keyboard<S, const W: usize, const H: usize>
{
    scanner: S,
    map: [[KeyCode; W]; H],
}

impl<'a, S, const W: usize, const H: usize> Keyboard<S, W, H>
where
    S: KeyScan<W, H> 
{
    pub fn new(scanner: S, map: [[KeyCode; W]; H]) -> Self {
        Self {
            scanner,
            map,
        }
    }

    pub async fn run<D: Driver<'a>>(
        mut self, 
        config: Config<'a>, 
        driver: D, 
        bufs: &'a mut Buffers<'a>
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
            loop {
                let mut scan = [[false; W]; H]; 
                self.scanner.scan(&mut scan).await;
                let report = self.report(scan);
                let _ = writer.write(report.as_slice()).await;
            }
        };
    
        let out_fut = reader.run(false, &mut bufs.request_handler);
    
        let _ = join3(usb_fut, in_fut, out_fut).await;

        panic!()
    }

    fn report(&self, scan: [[bool; W]; H]) -> Report {
        let mut modifiers = 0;
        let mut keycodes = [KeyCode::NoEvent; 6];
        let mut count = 0;

        'outer: for (x, row) in scan.into_iter().enumerate() {
            for (y, key) in row.into_iter().enumerate() {
                if !key {
                    continue;
                }

                let code = self.map[y][x];

                if let Some(mask) = code.modifier_mask() {
                    modifiers |= mask;
                    continue;
                }

                if count >= keycodes.len() {
                    keycodes.fill(KeyCode::RollOverError);
                    break 'outer;
                }
    
                keycodes[count] = code;
                count += 1;
            }
        }

        Report::new(modifiers, keycodes)
    }
}