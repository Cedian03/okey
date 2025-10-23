use embassy_nrf::{
    bind_interrupts,
    gpio::{Input, Level, Output, OutputDrive, Pull},
    peripherals::USBD,
    usb::{self, Driver, vbus_detect::HardwareVbusDetect},
};

use crate::Init;

bind_interrupts!(struct Irqs {
    USBD => usb::InterruptHandler<USBD>;
    CLOCK_POWER => usb::vbus_detect::InterruptHandler;
});

pub fn init<'d>() -> Init<Driver<'d, HardwareVbusDetect>, Output<'d>, Input<'d>> {
    let p = embassy_nrf::init(Default::default());

    Init {
        driver: Driver::new(p.USBD, Irqs, HardwareVbusDetect::new(Irqs)),
        cols: [
            Output::new(p.P0_00, Level::Low, OutputDrive::Standard),
            Output::new(p.P0_01, Level::Low, OutputDrive::Standard),
        ],
        rows: [
            Input::new(p.P0_02, Pull::Down),
            Input::new(p.P0_03, Pull::Down),
        ],
    }
}
