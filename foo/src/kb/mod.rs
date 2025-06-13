#![allow(unused, private_bounds)]

use std::marker::PhantomData;

mod builder;
mod layers;
mod state;

pub trait Scan<const W: usize, const H: usize> {
    fn scan(&mut self) -> [[bool; W]; H];
}

struct DummyScanner<const W: usize, const H: usize>;

impl<const W: usize, const H: usize> Scan<W, H> for DummyScanner<W, H> {
    fn scan(&mut self) -> [[bool; W]; H] {
        todo!()
    }
}

pub trait Map<const W: usize, const H: usize> {
    type Action;

    fn get(&mut self, x: usize, y: usize) -> Option<Self::Action>;
}

pub struct MatrixMap<A, const W: usize, const H: usize>([[A; W]; H]);

impl<A, const W: usize, const H: usize> Map<W, H> for MatrixMap<A, W, H> {
    type Action = A;

    fn get(&mut self, x: usize, y: usize) -> Option<A> {
        todo!()
    }
}

pub trait Handler<S> {
    type Action;

    fn handle(&mut self, state: S, action: Self::Action);
}

#[derive(Clone, Copy, Debug)]
enum DummyAction {
    Foo,
    Bar,
}

struct DummyHandler;

impl<S> Handler<S> for DummyHandler {
    type Action = DummyAction;

    fn handle(&mut self, state: S, action: Self::Action) {
        todo!()
    }
}

enum InterestingAction<A> {
    Inner(A),
    Hmm,
    Aha,
}

struct InterestingState;

struct InterestingHandlerThatRequiresInterestingState<I> {
    inner: I,
}

impl<I: Handler<S>, S: Retrieve<InterestingState>> Handler<S>
    for InterestingHandlerThatRequiresInterestingState<I>
{
    type Action = InterestingAction<I::Action>;

    fn handle(&mut self, state: S, action: Self::Action) {
        todo!()
    }
}

trait Retrieve<T> {
    fn retrieve(&self) -> &T;
}

impl<T> Retrieve<T> for T {
    fn retrieve(&self) -> &T {
        self
    }
}

trait RetrieveMut<T> {
    fn retrieve_mut(&mut self) -> &mut T;
}

impl<T> RetrieveMut<T> for T {
    fn retrieve_mut(&mut self) -> &mut T {
        self
    }
}

struct MapAndInterestingState<A, const W: usize, const H: usize> {
    map: MatrixMap<A, W, H>,
    interesting_state: InterestingState,
}

impl<A, const W: usize, const H: usize> Retrieve<MatrixMap<A, W, H>>
    for MapAndInterestingState<A, W, H>
{
    fn retrieve(&self) -> &MatrixMap<A, W, H> {
        &self.map
    }
}

impl<A, const W: usize, const H: usize> Retrieve<InterestingState>
    for MapAndInterestingState<A, W, H>
{
    fn retrieve(&self) -> &InterestingState {
        &self.interesting_state
    }
}

impl<A, const W: usize, const H: usize> RetrieveMut<MatrixMap<A, W, H>>
    for MapAndInterestingState<A, W, H>
{
    fn retrieve_mut(&mut self) -> &mut MatrixMap<A, W, H> {
        &mut self.map
    }
}

impl<A, const W: usize, const H: usize> RetrieveMut<InterestingState>
    for MapAndInterestingState<A, W, H>
{
    fn retrieve_mut(&mut self) -> &mut InterestingState {
        &mut self.interesting_state
    }
}

pub struct Keyboard<SCANNER, HANDLER, STATE, MAP, const W: usize, const H: usize>
where
    SCANNER: Scan<W, H>,
    HANDLER: Handler<STATE>,
    STATE: Retrieve<MAP>,
    MAP: Map<W, H, Action = HANDLER::Action>,
{
    scanner: SCANNER,
    handler: HANDLER,
    state: STATE,
    _data: PhantomData<MAP>,
}

impl<SCANNER, HANDLER, STATE, MAP, const W: usize, const H: usize>
    Keyboard<SCANNER, HANDLER, STATE, MAP, W, H>
where
    SCANNER: Scan<W, H>,
    HANDLER: Handler<STATE>,
    STATE: Retrieve<MAP>,
    MAP: Map<W, H, Action = HANDLER::Action>,
{
    fn new(scanner: SCANNER, handler: HANDLER, state: STATE) -> Self {
        Self {
            scanner,
            handler,
            state,
            _data: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_without_() {
        let mut _keyboard: Keyboard<_, _, _, MatrixMap<_, 2, 2>, 2, 2> = Keyboard::new(
            DummyScanner,
            InterestingHandlerThatRequiresInterestingState {
                inner: DummyHandler,
            },
            // DummyHandler,
            MapAndInterestingState {
                map: MatrixMap([
                    [InterestingAction::Hmm, InterestingAction::Aha],
                    [
                        InterestingAction::Inner(DummyAction::Bar),
                        InterestingAction::Inner(DummyAction::Bar),
                    ],
                ]),
                interesting_state: InterestingState,
            },
        );

        // let _: &mut MatrixMap<_, 2, 2> = _keyboard.state.retrieve_mut();
    }
}
