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

use crate::{scan::Scan, Event, Keyboard};

pub struct UsbInterface<'a, T: Driver<'a>> {
    device: UsbDevice<'a, T>,
    _reader: HidReader<'a, T, 1>,
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

        let (_reader, writer) = HidReaderWriter::new(
            &mut builder, 
            &mut state.hid_state, 
            hid_config
        ).split();

        let device = builder.build();

        Self {
            device,
            _reader,
            writer,
        }
    }

    pub async fn run<S: Scan<W, H>, const W: usize, const H: usize, const D: usize>(self, mut board: Keyboard<S, W, H, D>) {
        let Self { mut device, _reader, mut writer } = self;

        let key_fut = async {
            let mut scan = &mut [[false; W]; H];
            let mut last_scan = &mut [[false; W]; H];
            
            let mut report = Report::default();

            loop {
                board.scanner.scan(scan).await; 
    
                for y in 0..H {
                    for x in 0..W {
                        if scan[y][x] != last_scan[y][x] {
                            board.handle_key_event(x, y, Event::new(scan[y][x]), &mut report);
                        }
                    }
                }
    
                let _ = writer.write(report.as_slice()).await;
    
                core::mem::swap(&mut scan, &mut last_scan);
            }
        };

        join(device.run(), key_fut).await;
    } 
}
