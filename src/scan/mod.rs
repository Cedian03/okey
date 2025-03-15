pub mod debounce;

mod direct;
mod col2row;
mod row2col;

pub use direct::Direct;
pub use col2row::Col2Row;
pub use row2col::Row2Col;

pub trait Scan<const W: usize, const H: usize> {
    fn scan(&mut self, buf: &mut [[bool; W]; H]) -> impl core::future::Future<Output = ()>;
}