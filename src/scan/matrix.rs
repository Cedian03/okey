use core::convert::Infallible;

use embassy_time::Timer;
use embedded_hal::digital::{InputPin, OutputPin};

use super::KeyScan;

pub struct Matrix<C, R, const W: usize, const H: usize> {
    cols: [C; W],
    rows: [R; H],
}

impl<C, R, const W: usize, const H: usize> Matrix<C, R, W, H> {
    pub fn new(cols: [C; W], rows: [R; H]) -> Self {
        Self {
            cols, 
            rows,
        }
    }
}

impl<C, R, const W: usize, const H: usize> KeyScan<W, H> for Matrix<C, R, W, H>
where 
    C: OutputPin<Error = Infallible>,
    R: InputPin<Error = Infallible>, 
{
    async fn scan(&mut self, buf: &mut [[bool; W]; H]) {
        for (cpin, row) in self.cols.iter_mut().zip(buf.iter_mut()) {
            cpin.set_high().unwrap();
            Timer::after_micros(30).await;

            for (rpin, dst) in self.rows.iter_mut().zip(row.iter_mut()) {
                *dst = rpin.is_high().unwrap();
            }
            
            cpin.set_low().unwrap();
            Timer::after_micros(30).await;
        }
    }
}
