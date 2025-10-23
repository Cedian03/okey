pub mod debounce;

mod col2row;
mod direct;
mod row2col;

pub use col2row::Col2Row;
pub use direct::Direct;
pub use row2col::Row2Col;

pub trait Scan<const W: usize, const H: usize> {
    fn scan(&mut self, buf: &mut [[bool; W]; H]) -> impl core::future::Future<Output = ()>;
}
