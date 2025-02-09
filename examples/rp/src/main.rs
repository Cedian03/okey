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

    let cols = [
        Output::new(p.PIN_0, Level::Low),
        Output::new(p.PIN_1, Level::Low),
    ];

    let rows = [
        Input::new(p.PIN_10, Pull::Down),
        Input::new(p.PIN_11, Pull::Down),
    ];

    let map = [
        [
            [Some(Action::Key(Key::KeyboardA)),             Some(Action::Key(Key::KeyboardB))],
            [Some(Action::Modifier(Modifier::LeftControl)), Some(Action::ToggleLayer(1))     ],
        ],
        [
            [Some(Action::Key(Key::KeyboardX)),             Some(Action::Key(Key::KeyboardY))],
            [Some(Action::Modifier(Modifier::LeftShift)),   Some(Action::ToggleLayer(1))     ],
        ],
    ];

    let config = Config::new(0xC3DD, 0x0000);
    
    let keyboard = Keyboard::new(Matrix::col2row(cols, rows), map);

    keyboard.run(config, driver, &mut Default::default()).await;
}
