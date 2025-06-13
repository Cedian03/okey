#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts, gpio::{Input, Level, Output, Pull}, i2c::{I2c, InterruptHandler as I2cInterruptHandler}, i2c_slave::{self, Command, I2cSlave}, peripherals::I2C1
};

use okey::prelude::*;

use panic_probe as _;

bind_interrupts!(struct Irqs {
    I2C1_IRQ => I2cInterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut driver = I2cSlave::new(
        p.I2C1,
        p.PIN_11,
        p.PIN_10,
        Irqs,
        Default::default(),
    );

    let scanner = {
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

    async {
        loop {
            let mut buf = [0];
            match driver.listen(&mut buf).await.unwrap() {
                Command::WriteRead(n) => {
                    match n {
                        0 => { let _ = driver.respond_to_read(&[2, 2]).await; },
                        1 => { let _ = driver.respond_to_read(&[2, 0, 0, 1, 1]).await; }, 
                        _ => unimplemented!()
                    }
                }
                _ => unimplemented!()
            };
        }
    }.await
}
