mod code;
mod config;
mod handlers;
mod report;

use embassy_futures::join::join;

use embassy_usb::{
    class::hid::{HidReader, HidReaderWriter, HidWriter, State}, 
    driver::Driver, 
    Builder, 
    UsbDevice
};

pub use code::Code;
pub use config::Config;
pub use handlers::{OkeyDeviceHandler, OkeyRequestHandler};
pub use report::{Report, ReportError};

use crate::{scan::KeyScan, Keyboard};

// TODO: Make this is actually safe or find another way.
static mut BUFFERS: Option<Buffers> = Some(unsafe {
    Buffers::new(
        &mut CONFIG_DESCRIPTOR_BUF,
        &mut BOS_DESCRIPTOR_BUF,
        &mut MSOS_DESCRIPTOR_BUF,
        &mut CONTROL_BUF,
    )
});

static mut CONFIG_DESCRIPTOR_BUF: [u8; 256] = [0; 256];
static mut BOS_DESCRIPTOR_BUF: [u8; 256] = [0; 256];
static mut MSOS_DESCRIPTOR_BUF: [u8; 256] = [0; 256];
static mut CONTROL_BUF: [u8; 64] = [0; 64];

pub struct UsbInterface<'a, T: Driver<'a>> {
    device: UsbDevice<'a, T>,
    reader: HidReader<'a, T, 1>,
    writer: HidWriter<'a, T, 8>,
}

impl<'a, T: Driver<'a>> UsbInterface<'a, T> {
    pub fn new(
        driver: T, 
        config: Config<'a>, 
        hid_state: &'a mut State<'a>,  // TODO: Get rid of this.
    ) -> Self {
        let (usb_config, hid_config) = config.split();

        let buffers = Buffers::take();

        let mut builder = Builder::new(
            driver, 
            usb_config, 
            buffers.config_descriptor_buf,
            buffers.bos_descriptor_buf,
            buffers.msos_descriptor_buf,
            buffers.control_buf,
        );

        let (reader, writer) = HidReaderWriter::new(
            &mut builder, 
            hid_state, 
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

struct Buffers<'a> {
    config_descriptor_buf: &'a mut [u8],
    bos_descriptor_buf: &'a mut [u8],
    msos_descriptor_buf: &'a mut [u8],
    control_buf: &'a mut [u8],
}

impl<'a> Buffers<'a> {
    const fn new(
        config_descriptor_buf: &'a mut [u8],
        bos_descriptor_buf: &'a mut [u8],
        msos_descriptor_buf: &'a mut [u8],
        control_buf: &'a mut [u8],
    ) -> Self {
        Self {
            config_descriptor_buf,
            bos_descriptor_buf,
            msos_descriptor_buf,
            control_buf,
        }
    }
}

impl Buffers<'static> {
    const fn take() -> Self {
        // TODO: Make this is actually safe or find another way.
        unsafe { BUFFERS.take().unwrap() }
    }
}