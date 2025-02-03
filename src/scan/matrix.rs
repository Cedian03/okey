use core::{convert::Infallible, marker::PhantomData};

use embassy_time::Timer;
use embedded_hal::digital::{InputPin, OutputPin};

use super::KeyScan;

pub struct Matrix<D: DiodDirection, const W: usize, const H: usize> {
    cols: [D::Cols; W],
    rows: [D::Rows; H],
    _data: PhantomData<D>,
}

impl<C, R, const W: usize, const H: usize> Matrix<COL2ROW<C, R>, W, H> 
where 
    C: OutputPin<Error = Infallible>,
    R: InputPin<Error = Infallible>, 
{
    pub fn col2row(cols: [C; W], rows: [R; H]) -> Self {
        Self {
            cols,
            rows,
            _data: PhantomData
        }
    }
}

impl<C, R, const W: usize, const H: usize> Matrix<ROW2COL<C, R>, W, H> 
where 
    C: InputPin<Error = Infallible>, 
    R: OutputPin<Error = Infallible>,
{
    pub fn row2col(cols: [C; W], rows: [R; H]) -> Self {
        Self {
            cols,
            rows,
            _data: PhantomData
        }
    }
}

impl<C, R, const W: usize, const H: usize> KeyScan<W, H> for Matrix<COL2ROW<C, R>, W, H>
where 
    C: OutputPin<Error = Infallible>,
    R: InputPin<Error = Infallible>, 
{
    async fn scan(&mut self, buf: &mut [[bool; W]; H]) {
        for (cpin, row) in self.cols.iter_mut().zip(buf.iter_mut()) {
            cpin.set_high().unwrap();
            Timer::after_micros(30).await;

            for (rpin, dst) in self.rows.iter_mut().zip(row.iter_mut()) {
                *dst = rpin.is_high().unwrap();
            }
            
            cpin.set_low().unwrap();
            Timer::after_micros(30).await;
        }
    }
}

impl<C, R, const W: usize, const H: usize> KeyScan<W, H> for Matrix<ROW2COL<C, R>, W, H>
where 
    C: InputPin<Error = Infallible>, 
    R: OutputPin<Error = Infallible>,
{
    async fn scan(&mut self, buf: &mut [[bool; W]; H]) {
        for (rpin, row) in self.rows.iter_mut().zip(buf.iter_mut()) {
            rpin.set_high().unwrap();
            Timer::after_micros(30).await;

            for (cpin, dst) in self.cols.iter_mut().zip(row.iter_mut()) {
                *dst = cpin.is_high().unwrap();
            }
            
            rpin.set_low().unwrap();
            Timer::after_micros(30).await;
        }
    }
}

trait DiodDirection {
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
