use core::convert::Infallible;

use embedded_hal::digital::InputPin;

use super::Scan;

pub struct Direct<P, const N: usize> {
    pins: [P; N]
}

impl<P, const N: usize> Scan<N, 1> for Direct<P, N>
where 
    P: InputPin<Error = Infallible>
{
    async fn scan(&mut self, buf: &mut [[bool; N]; 1]) {
        for (pin, dst) in self.pins.iter_mut().zip(buf[0].iter_mut()) {
            *dst = pin.is_high().unwrap();
        }
    }
}