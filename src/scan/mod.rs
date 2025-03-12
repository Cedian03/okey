pub mod debounce;

mod direct;
mod matrix;

pub use direct::Direct;
pub use matrix::Matrix;

pub trait Scan<const W: usize, const H: usize> {
    fn scan(&mut self, buf: &mut [[bool; W]; H]) -> impl core::future::Future<Output = ()>;
}