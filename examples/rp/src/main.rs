#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output, Pull},
    peripherals::USB,
    usb::{Driver, InterruptHandler},
};

use okey::prelude::*;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let driver = Driver::new(p.USB, Irqs);
    let config = Config::default();
    let mut state = State::default();
    let interface = UsbInterface::new(driver, config, &mut state);

    let scanner = {
        let cols = [
            Output::new(p.PIN_0, Level::Low),
            Output::new(p.PIN_1, Level::Low),
        ];

        let rows = [
            Input::new(p.PIN_10, Pull::Down),
            Input::new(p.PIN_11, Pull::Down),
        ];

        Col2Row::debounced(cols, rows)
    };

    let map = {
        use okey::codes::*;

        [
            [
                [KC_A,    KC_ENTR],
                [KC_B,    TG(1)  ],
            ],
            [
                [KC_1,    KC_LCTL],
                [KC_2,    _______],
            ],
        ]
    };

    Keyboard::new(scanner, map, interface).run().await
}
