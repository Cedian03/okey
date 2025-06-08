use crate::scan::Scan;

use super::DEBOUNCE_COUNT;

const DEBOUNCE_MASK: u16 = (1 << DEBOUNCE_COUNT) - 1;

pub struct Simple<S, const W: usize, const H: usize> {
    scanner: S,
    states: [[u16; W]; H],
}

impl<S, const W: usize, const H: usize> Simple<S, W, H> {
    pub const fn new(scanner: S) -> Self {
        Self {
            scanner,
            states: [[0; W]; H],
        }
    }
}

impl<S: Scan<W, H>, const W: usize, const H: usize> Scan<W, H> for Simple<S, W, H> {
    async fn scan(&mut self, buf: &mut [[bool; W]; H]) {
        self.scanner.scan(buf).await;

        for y in 0..H {
            for x in 0..W {
                self.states[y][x] = (self.states[y][x] << 1) | (buf[y][x] as u16);
                buf[y][x] = (self.states[y][x] & DEBOUNCE_MASK) == DEBOUNCE_MASK;
            }
        }
    }
}
