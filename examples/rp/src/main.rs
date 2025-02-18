#![no_std]
#![no_main]

use embassy_executor::Spawner;

use embassy_rp::{
    bind_interrupts, 
    gpio::{Input, Level, Output, Pull}, 
    peripherals::USB, 
    usb::{Driver, InterruptHandler},
};

use okey::prelude::*;

use panic_probe as _;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let d = Driver::new(p.USB, Irqs);

    let config = Config::default()
        .pid(0xC3DD)
        .vid(0x1337);

    let mut hid_state = Default::default(); // TODO: Get rid of this.
    
    let usb = UsbInterface::new(d, config, &mut hid_state);

    let matrix = {
        let cols = [
            Output::new(p.PIN_0, Level::Low),
            Output::new(p.PIN_1, Level::Low),
        ];
    
        let rows = [
            Input::new(p.PIN_10, Pull::Down),
            Input::new(p.PIN_11, Pull::Down),
        ];

        Matrix::col2row(cols, rows)
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

    let board = Keyboard::new(matrix, map);

    usb.run(board).await;
}
