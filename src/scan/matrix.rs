use core::{convert::Infallible, marker::PhantomData};

use embassy_time::Timer;
use embedded_hal::digital::{InputPin, OutputPin};

use super::Scan;

pub struct Matrix<D: DiodDirection, const W: usize, const H: usize> {
    cols: [D::Cols; W],
    rows: [D::Rows; H],
    _d: PhantomData<D>,
}

impl<D: DiodDirection, const W: usize, const H: usize> Matrix<D, W, H> {
    fn new(cols: [D::Cols; W], rows: [D::Rows; H]) -> Self {
        Self {
            cols,
            rows,
            _d: PhantomData,
        }
    }
}

impl<C, R, const W: usize, const H: usize> Matrix<COL2ROW<C, R>, W, H> 
where 
    C: OutputPin<Error = Infallible>,
    R: InputPin<Error = Infallible>, 
{
    pub fn col2row(cols: [C; W], rows: [R; H]) -> Self {
        Self::new(cols, rows)
    }
}

impl<C, R, const W: usize, const H: usize> Matrix<ROW2COL<C, R>, W, H> 
where 
    C: InputPin<Error = Infallible>, 
    R: OutputPin<Error = Infallible>,
{
    pub fn row2col(cols: [C; W], rows: [R; H]) -> Self {
        Self::new(cols, rows)
    }
}

impl<C, R, const W: usize, const H: usize> Scan<W, H> for Matrix<COL2ROW<C, R>, W, H>
where 
    C: OutputPin<Error = Infallible>,
    R: InputPin<Error = Infallible>, 
{
    async fn scan(&mut self, buf: &mut [[bool; W]; H]) {
        for (col_pin, row) in self.cols.iter_mut().zip(buf.iter_mut()) {
            col_pin.set_high().unwrap();
            Timer::after_micros(30).await;

            for (row_pin, dst) in self.rows.iter_mut().zip(row.iter_mut()) {
                *dst = row_pin.is_high().unwrap();
            }
            
            col_pin.set_low().unwrap();
            Timer::after_micros(30).await;
        }
    }
}

impl<C, R, const W: usize, const H: usize> Scan<W, H> for Matrix<ROW2COL<C, R>, W, H>
where 
    C: InputPin<Error = Infallible>, 
    R: OutputPin<Error = Infallible>,
{
    async fn scan(&mut self, buf: &mut [[bool; W]; H]) {
        for (row_pin, row) in self.rows.iter_mut().zip(buf.iter_mut()) {
            row_pin.set_high().unwrap();
            Timer::after_micros(30).await;

            for (col_pin, dst) in self.cols.iter_mut().zip(row.iter_mut()) {
                *dst = col_pin.is_high().unwrap();
            }
            
            row_pin.set_low().unwrap();
            Timer::after_micros(30).await;
        }
    }
}

pub trait DiodDirection: private::Sealed {
    type Cols;
    type Rows;
}

pub struct COL2ROW<C, R> {
    _c: PhantomData<C>,
    _r: PhantomData<R>,
}

pub struct ROW2COL<C, R> {
    _c: PhantomData<C>,
    _r: PhantomData<R>,
}

impl<C, R> DiodDirection for COL2ROW<C, R> 
where 
    C: OutputPin<Error = Infallible>,
    R: InputPin<Error = Infallible>, 
{
    type Cols = C;    
    type Rows = R;    
}

impl<C, R> DiodDirection for ROW2COL<C, R> 
where 
    C: InputPin<Error = Infallible>, 
    R: OutputPin<Error = Infallible>,
{
    type Cols = C;    
    type Rows = R;    
}

mod private {
    use super::*;

    pub trait Sealed {}

    impl<C, R> Sealed for COL2ROW<C, R> {}
    impl<C, R> Sealed for ROW2COL<C, R> {}
}