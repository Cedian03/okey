mod builder;
mod state;

pub trait Scan<const W: usize, const H: usize> {
    fn scan(&mut self) -> [[bool; W]; H];
}

pub trait Map<A, const W: usize, const H: usize> {
    fn get(&mut self, x: usize, y: usize) -> Option<A>;
}

pub struct Matrix<A, const W: usize, const H: usize, const D: usize>([[[Option<A>; W]; H]; D]);

impl<A, const W: usize, const H: usize, const D: usize> Map<A, W, H> for Matrix<A, W, H, D> {
    fn get(&mut self, x: usize, y: usize) -> Option<A> {
        todo!()
    }
}

pub trait Handler {
    type Action;

    fn handle(&mut self, action: Self::Action);
}

trait Retrieve<T> {
    fn retrieve(&self) -> &T;
}

trait RetrieveMut<T> {
    fn retrieve_mut(&mut self) -> &mut T;
}

pub struct Keyboard<SCANNER, MAP, HANDLER, const W: usize, const H: usize>
where
    SCANNER: Scan<W, H>,
    MAP: Map<HANDLER::Action, W, H>,
    HANDLER: Handler,
{
    scanner: SCANNER,
    map: MAP,
    handler: HANDLER,
}

impl<SCANNER, MAP, HANDLER, const W: usize, const H: usize> Keyboard<SCANNER, MAP, HANDLER, W, H> 
where 
    SCANNER: Scan<W, H>,
    MAP: Map<HANDLER::Action, W, H>,
    HANDLER: Handler,
{
    fn new(scanner: SCANNER, map: MAP, handler: HANDLER) -> Self {
        Self {
            scanner,
            map,
            handler,
        }
    }
}

