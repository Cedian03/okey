#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output, Pull},
    peripherals::USB,
    usb::{Driver, InterruptHandler},
};

use okey::*;

use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let _keyboard = KeyboardBuilder::new()
        .scanner({
            let cols = [
                Output::new(p.PIN_0, Level::Low),
                Output::new(p.PIN_1, Level::Low),
            ];

            let rows = [
                Input::new(p.PIN_10, Pull::Down),
                Input::new(p.PIN_11, Pull::Down),
            ];

            Col2Row::debounced(cols, rows)
        })
        .handler(UsbHandler)
        .wrap_handler::<FooHandler>()
        .wrap_handler::<BarHandler>()
        .map(
            #[rustfmt::skip] 
            Matrix([
                [
                    [Some(BarAction::Inner(FooAction::Inner(UsbAction::A))), Some(BarAction::Inner(FooAction::Inner(UsbAction::B)))],
                    [Some(BarAction::Inner(FooAction::Inner(UsbAction::C))), None                                                  ],
                ],
                [
                    [Some(BarAction::Bar1),                                  Some(BarAction::Inner(FooAction::Foo1))               ],
                    [Some(BarAction::Bar2),                                  Some(BarAction::Inner(FooAction::Foo2))               ],
                ]
            ])
        )
        .build();
}
