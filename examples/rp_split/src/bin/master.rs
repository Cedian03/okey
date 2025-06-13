#![no_std]
#![no_main]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output, Pull},
    peripherals::{USB, I2C1},
    i2c::{I2c, InterruptHandler as I2cInterruptHandler},
    usb::{Driver, InterruptHandler as UsbInterruptHandler},
};

use okey::{prelude::*, scan::split::{I2cScanner, SplitScanner}};

use panic_probe as _;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => UsbInterruptHandler<USB>;
    I2C1_IRQ => I2cInterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let driver = Driver::new(p.USB, Irqs);
    let config = Config::default();
    let mut state = State::default();
    let interface = UsbInterface::new(driver, config, &mut state);

    let scanner = {
        let left = {
            let cols = [
                Output::new(p.PIN_0, Level::Low),
                Output::new(p.PIN_1, Level::Low),
            ];

            let rows = [
                Input::new(p.PIN_5, Pull::Down),
                Input::new(p.PIN_6, Pull::Down),
            ];

            Col2Row::debounced(cols, rows)
        };

        let right = {
            let i2c = I2c::new_async(
                p.I2C1,
                p.PIN_11,
                p.PIN_10,
                Irqs,
                Default::default(),
            );

            I2cScanner::<_, 2, 2>::new(i2c)
        };

        SplitScanner::new(left, right)
    };

    let map = {
        use okey::codes::*;

        [
            [
                [KC_A, KC_B, KC_1, KC_2],
                [KC_B, KC_D, KC_3, KC_4],
            ],
        ]
    };

    Keyboard::new(scanner, map, interface).run().await
}
