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
    
    let driver = Driver::new(p.USB, Irqs);

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
                [KC_A,    KC_B   ],
                [KC_C,    TG(1)  ],
            ],
            [
                [KC_1,    KC_2   ],
                [KC_LCRL, _______],
            ],
        ]
    };

    let config = Config::new(0xC3DD, 0x0000);
    
    let keyboard = Keyboard::new(matrix, map);

    keyboard.run(config, driver, &mut Default::default()).await;
}
