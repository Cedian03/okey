use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output, Pull},
    peripherals::USB,
    usb::{Driver, InterruptHandler},
};

use crate::Init;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

pub fn init<'d>() -> Init<Driver<'d, USB>, Output<'d>, Input<'d>> {
    let p = embassy_rp::init(Default::default());

    Init {
        driver: Driver::new(p.USB, Irqs),
        cols: [
            Output::new(p.PIN_2, Level::Low),
            Output::new(p.PIN_3, Level::Low),
            Output::new(p.PIN_4, Level::Low),
        ],
        rows: [
            Input::new(p.PIN_5, Pull::Down),
            Input::new(p.PIN_23, Pull::Down),
            Input::new(p.PIN_21, Pull::Down),
        ],
    }
}
