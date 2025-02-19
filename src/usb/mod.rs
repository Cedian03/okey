mod code;
mod config;
mod handlers;
mod report;
mod state;

use embassy_futures::join::join;

use embassy_usb::{
    class::hid::{HidReader, HidReaderWriter, HidWriter}, 
    driver::Driver, 
    Builder, 
    UsbDevice
};

pub use code::Code;
pub use config::Config;
pub use handlers::{OkeyDeviceHandler, OkeyRequestHandler};
pub use report::{Report, ReportError};
pub use state::State;

use crate::{scan::KeyScan, Keyboard};

pub struct UsbInterface<'a, T: Driver<'a>> {
    device: UsbDevice<'a, T>,
    reader: HidReader<'a, T, 1>,
    writer: HidWriter<'a, T, 8>,
}

impl<'a, T: Driver<'a>> UsbInterface<'a, T> {
    pub fn new(driver: T, config: Config<'a>, state: &'a mut State<'a>) -> Self {
        let (usb_config, hid_config) = config.split();

        let mut builder = Builder::new(
            driver, 
            usb_config, 
            &mut state.config_descriptor_buf,
            &mut state.bos_descriptor_buf,
            &mut state.msos_descriptor_buf,
            &mut state.control_buf,
        );

        let (reader, writer) = HidReaderWriter::new(
            &mut builder, 
            &mut state.hid_state, 
            hid_config
        ).split();

        let device = builder.build();

        Self {
            device,
            reader,
            writer,
        }
    }

    pub async fn run<S: KeyScan<W, H>, const W: usize, const H: usize, const D: usize>(self, mut board: Keyboard<S, W, H, D>) {
        let Self { mut device, reader: _, writer: mut hid_writer } = self;

        let key_fut = async {
            let mut scan = &mut [[false; W]; H];
            let mut last_scan = &mut [[false; W]; H];
            
            let mut report = Report::default();

            loop {
                board.scanner.scan(scan).await; 
    
                for y in 0..H {
                    for x in 0..W {
                        if scan[y][x] != last_scan[y][x] {
                            board.handle_key_event(x, y, scan[y][x].into(), &mut report);
                        }
                    }
                }
    
                let _ = hid_writer.write(report.as_slice()).await;
    
                core::mem::swap(&mut scan, &mut last_scan);
            }
        };

        join(device.run(), key_fut).await;
    } 
}
