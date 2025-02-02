mod matrix;

pub use matrix::Matrix;

pub trait KeyScan<const W: usize, const H: usize> {
    async fn scan(&mut self, buf: &mut [[bool; W]; H]);
}