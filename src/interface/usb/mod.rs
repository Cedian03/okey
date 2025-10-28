pub mod qmk_key_codes;

mod config;
mod handlers;
mod key_codes;
mod report;
mod state;

use core::cell::Cell;

use critical_section::Mutex;
use embassy_futures::join::join;
use embassy_usb::{
    Builder, UsbDevice,
    class::hid::{HidReader, HidReaderWriter, HidWriter},
    driver::Driver,
};
use static_cell::StaticCell;

use crate::{debug, interface::usb::handlers::OkeyDeviceHandler, trace};

use report::Report;

use super::{Handler, Interface};

pub use config::Config;
pub use key_codes::KeyCode;
pub use report::Modifiers;
pub use state::State;

static SHARED_REPORT: Mutex<Cell<Report>> = Mutex::new(Cell::new(Report::new()));
static WAS_REPORT_SENT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

static DEVICE_HANDLER: StaticCell<OkeyDeviceHandler> = StaticCell::new();

pub struct UsbInterface<'d, D: Driver<'d>> {
    device: UsbDevice<'d, D>,
    _reader: HidReader<'d, D, 1>,
    writer: HidWriter<'d, D, 8>,
}

impl<'d, D: Driver<'d>> UsbInterface<'d, D> {
    pub fn new(driver: D, config: Config<'d>, state: &'d mut State<'d>) -> Self {
        debug!("Building USB interface with config: {}", config);
        let (usb_config, hid_config) = config.split();

        let mut builder = Builder::new(
            driver,
            usb_config,
            &mut state.config_descriptor_buf,
            &mut state.bos_descriptor_buf,
            &mut state.msos_descriptor_buf,
            &mut state.control_buf,
        );

        // TODO: This handler does nothing except log some info, that might be an issue.
        builder.handler(DEVICE_HANDLER.try_init_with(|| OkeyDeviceHandler).unwrap());

        let (_reader, writer) =
            HidReaderWriter::new(&mut builder, &mut state.hid_state, hid_config).split();

        let device = builder.build();

        Self {
            device,
            _reader,
            writer,
        }
    }
}

impl<'d, D: Driver<'d>> Interface for UsbInterface<'d, D> {
    type Handler = UsbHandler;

    fn start(mut self) -> (Self::Handler, impl Future) {
        let fut1 = async move {
            debug!("Running USB device...");
            self.device.run().await;
        };

        let fut2 = async move {
            debug!("Running USB report writer...");
            loop {
                self.writer.ready().await;

                let report = critical_section::with(|cs| {
                    WAS_REPORT_SENT.borrow(cs).set(true);
                    SHARED_REPORT.borrow(cs).get()
                });

                let _ = self.writer.write(report.as_slice()).await;
            }
        };

        (UsbHandler::new(), join(fut1, fut2))
    }
}

pub struct UsbHandler {
    persistent_report: Report,
    report: Report,
}

impl UsbHandler {
    const fn new() -> Self {
        Self {
            persistent_report: Report::new(),
            report: Report::new(),
        }
    }
}

impl Handler for UsbHandler {
    fn register(&mut self, code: KeyCode) {
        if self.report.add(code).is_ok() {
            self.persistent_report.add(code).unwrap();
        }
    }

    fn temp_register(&mut self, code: KeyCode) {
        let _ = self.report.add(code);
    }

    fn unregister(&mut self, code: KeyCode) {
        let _ = self.persistent_report.remove(code);
    }

    fn flush(&mut self) {
        trace!("Flushing USB report: {}", self.report);

        let sent = critical_section::with(|cs| {
            SHARED_REPORT.borrow(cs).set(self.report);
            WAS_REPORT_SENT.borrow(cs).replace(false)
        });

        if sent {
            self.report = self.persistent_report;
        }
    }
}
