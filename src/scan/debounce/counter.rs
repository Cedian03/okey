use crate::scan::Scan;

use super::DEBOUNCE_COUNT;

pub struct Counter<S, const W: usize, const H: usize> {
    scanner: S,
    counts: [[u16; W]; H],
    states: [[bool; W]; H],
}

impl<S, const W: usize, const H: usize> Counter<S, W, H> {
    pub const fn new(scanner: S) -> Self {
        Self {
            scanner,
            counts: [[DEBOUNCE_COUNT as u16; W]; H],
            states: [[false; W]; H],
        }
    }
}

impl<S: Scan<W, H>, const W: usize, const H: usize> Scan<W, H> for Counter<S, W, H> {
    async fn scan(&mut self, buf: &mut [[bool; W]; H]) {
        self.scanner.scan(buf).await;

        for y in 0..H {
            for x in 0..W {
                let raw_state = buf[y][x];
                let cur_state = self.states[y][x];

                if raw_state == cur_state {
                    self.counts[y][x] = DEBOUNCE_COUNT as u16;
                } else {
                    self.counts[y][x] -= 1;
                    if self.counts[y][x] == 0 {
                        self.states[y][x] = raw_state;
                        self.counts[y][x] = DEBOUNCE_COUNT as u16;
                    }
                }
            }
        }
    }
}
