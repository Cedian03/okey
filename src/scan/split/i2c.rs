use embedded_hal_async::i2c::{I2c, SevenBitAddress};

use super::Scan;

const ADDRESS: SevenBitAddress  = 0x10;

const PULL: u8 = 1;

pub struct I2cScanner<D: I2c, const W: usize, const H: usize> {
    driver: D,
}

impl<D: I2c, const W: usize, const H: usize> I2cScanner<D, W, H> {
    pub fn new(driver: D) -> Self {
        Self { driver }
    }
}

impl<D: I2c, const W: usize, const H: usize> Scan<W, H> for I2cScanner<D, W, H> {
    async fn scan(&mut self, buf: &mut [[bool; W]; H]) {
        let mut i2c_buf = [0; 2];
        self.driver
            .write_read(ADDRESS, &[PULL], &mut i2c_buf[0..1])
            .await
            .unwrap();

        let key_count = i2c_buf[0];

        *buf = [[false; W]; H];

        for _ in 0..key_count {
            self.driver.read(ADDRESS, &mut i2c_buf).await.unwrap();
            let [x, y] = i2c_buf;

            assert!(x < W as u8 && y < H as u8);

            buf[y as usize][x as usize] = true;
        }
    }
}
