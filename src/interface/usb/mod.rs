mod code;
mod config;
mod handlers;
mod report;
mod state;

use embassy_usb::{
    class::hid::{HidReader, HidReaderWriter, HidWriter}, 
    driver::Driver, 
    Builder, 
    UsbDevice
};

use super::{Interface, Handler};

pub use code::Code;
pub use config::Config;
pub use handlers::{OkeyDeviceHandler, OkeyRequestHandler};
pub use report::{Report, ReportError};
pub use state::State;

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
}

impl<'d, D: Driver<'d>> Interface for UsbInterface<'d, D> {
    type Handler = UsbHandler<'d, D>;

    fn start(mut self) -> (Self::Handler, impl Future) {
        (
            UsbHandler::new(self.writer),
            async move { self.device.run().await }
        )
    }
}

pub struct UsbHandler<'d, D: Driver<'d>> {
    writer: HidWriter<'d, D, 8>,
    report: Report,
    temp_report: Report,
}

impl<'d, D: Driver<'d>> UsbHandler<'d, D> {
    const fn new(writer: HidWriter<'d, D, 8>) -> Self {
        Self {
            writer,
            report: Report::new(),
            temp_report: Report::new(),
        }
    }
}

impl<'d, D: Driver<'d>> Handler for UsbHandler<'d, D> {
    fn register_code(&mut self, code: Code) {
        if self.temp_report.add(code).is_ok() {
            let _ = self.report.add(code);
        }
    }

    fn unregister_code(&mut self, code: Code) {
        let _ = self.report.remove(code);
    }

    fn temp_register_code(&mut self, code: Code) {
        let _ = self.temp_report.add(code);
    }

    async fn ready(&mut self) {
        self.writer.ready().await
    }

    async fn flush(&mut self) {
        let _ = self.writer.write(self.temp_report.as_slice()).await;
        self.temp_report = self.report;
    }
}