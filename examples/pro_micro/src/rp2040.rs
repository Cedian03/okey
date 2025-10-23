use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output, Pull},
    peripherals::USB,
    usb::{Driver, InterruptHandler},
};

use okey::prelude::*;

use crate::Init;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

pub fn init<'d>() -> Init<Driver<'d, USB>, Output<'d>, Input<'d>> {
    let p = embassy_rp::init(Default::default());

    Init {
        driver: Driver::new(p.USB, Irqs),
        cols: [
            Output::new(p.PIN_0, Level::Low),
            Output::new(p.PIN_1, Level::Low),
        ],
        rows: [
            Input::new(p.PIN_10, Pull::Down),
            Input::new(p.PIN_11, Pull::Down),
        ],
    }
}
