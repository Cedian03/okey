#![no_std]
#![no_main]

#[cfg(feature = "nrf52840")]
mod nrf52840;
#[cfg(feature = "rp2040")]
mod rp2040;

mod board {
    #[cfg(feature = "nrf52840")]
    pub use super::nrf52840::*;
    #[cfg(feature = "rp2040")]
    pub use super::rp2040::*;
}

use panic_probe as _;

use embassy_executor::Spawner;

use okey::prelude::*;

struct Init<D, C, R> {
    driver: D,
    cols: [C; 2],
    rows: [R; 2],
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let Init { driver, cols, rows } = board::init();

    let mut state = State::new();
    let interface = UsbInterface::new(driver, Config::default(), &mut state);

    let scanner = Col2Row::debounced(cols, rows);

    let map = {
        use okey::qmk_key_codes::*;

        [
            [[KC_A, KC_ENTR], [KC_B, TG(1)]],
            [[KC_1, KC_LCTL], [KC_2, _______]],
        ]
    };

    Keyboard::new(scanner, map, interface).run().await
}
