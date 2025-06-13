mod i2c;

use embassy_futures::join::join;

use super::Scan;

pub use i2c::I2cScanner;

pub struct SplitScanner<L, R, const LW: usize, const RW: usize, const H: usize>
where
    L: Scan<LW, H>,
    R: Scan<RW, H>,
{
    left: L,
    right: R,
}

impl<L, R, const LW: usize, const RW: usize, const H: usize> SplitScanner<L, R, LW, RW, H> 
where
    L: Scan<LW, H>,
    R: Scan<RW, H>,
{
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L, R, const LW: usize, const RW: usize, const H: usize> Scan<{ LW + RW }, H>
    for SplitScanner<L, R, LW, RW, H>
where
    L: Scan<LW, H>,
    R: Scan<RW, H>,
{
    async fn scan(&mut self, buf: &mut [[bool; LW + RW]; H]) {
        let mut left_buf = [[false; LW]; H];
        let mut right_buf = [[false; RW]; H];

        join(
            self.left.scan(&mut left_buf),
            self.right.scan(&mut right_buf),
        )
        .await;

        for y in 0..H {
            for x in 0..LW {
                buf[y][x] = left_buf[y][x]
            }

            for x in 0..RW {
                buf[y][LW + x] = left_buf[y][x]
            }
        }
    }
}
