use core::convert::Infallible;

use embassy_time::Timer;
use embedded_hal::digital::{InputPin, OutputPin};

use super::{Scan, debounce::debounce};

pub struct Col2Row<I, O, const W: usize, const H: usize> {
    cols: [O; W],
    rows: [I; H],
}

impl<I, O, const W: usize, const H: usize> Col2Row<I, O, W, H>
where
    I: InputPin<Error = Infallible>,
    O: OutputPin<Error = Infallible>,
{
    pub const fn new(cols: [O; W], rows: [I; H]) -> Self {
        Self { cols, rows }
    }

    pub const fn debounced(cols: [O; W], rows: [I; H]) -> impl Scan<W, H> {
        debounce(Self::new(cols, rows))
    }
}

impl<I, O, const W: usize, const H: usize> Scan<W, H> for Col2Row<I, O, W, H>
where
    I: InputPin<Error = Infallible>,
    O: OutputPin<Error = Infallible>,
{
    async fn scan(&mut self, buf: &mut [[bool; W]; H]) -> () {
        for (x, col_pin) in self.cols.iter_mut().enumerate() {
            col_pin.set_high().unwrap();
            Timer::after_micros(30).await;

            for (y, row_pin) in self.rows.iter_mut().enumerate() {
                buf[y][x] = row_pin.is_high().unwrap();
            }

            col_pin.set_low().unwrap();
            Timer::after_micros(30).await;
        }
    }
}
