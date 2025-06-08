use core::convert::Infallible;

use embassy_time::Timer;
use embedded_hal::digital::{InputPin, OutputPin};

use super::{Scan, debounce::debounce};

pub struct Row2Col<I, O, const W: usize, const H: usize> {
    rows: [O; H],
    cols: [I; W],
}

impl<I, O, const W: usize, const H: usize> Row2Col<I, O, W, H>
where
    I: InputPin<Error = Infallible>,
    O: OutputPin<Error = Infallible>,
{
    pub const fn new(rows: [O; H], cols: [I; W]) -> Self {
        Self { rows, cols }
    }

    pub const fn debounced(rows: [O; H], cols: [I; W]) -> impl Scan<W, H> {
        debounce(Self::new(rows, cols))
    }
}

impl<I, O, const W: usize, const H: usize> Scan<W, H> for Row2Col<I, O, W, H>
where
    I: InputPin<Error = Infallible>,
    O: OutputPin<Error = Infallible>,
{
    async fn scan(&mut self, buf: &mut [[bool; W]; H]) -> () {
        for y in 0..H {
            let row_pin = &mut self.rows[y];
            row_pin.set_high().unwrap();
            Timer::after_micros(30).await;

            for x in 0..W {
                let col_pin = &mut self.cols[x];
                buf[y][x] = col_pin.is_high().unwrap();
            }

            row_pin.set_low().unwrap();
            Timer::after_micros(30).await;
        }
    }
}
