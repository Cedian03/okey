use super::*;

#[derive(Clone, Copy, Debug)]
enum Opacity<T> {
    Opaque(T),
    Transparent,
}

impl<T> Into<Option<T>> for Opacity<T> {
    fn into(self) -> Option<T> {
        match self {
            Self::Opaque(x) => Some(x),
            Self::Transparent => None,
        }
    }
}

struct LayeredMap<A, const W: usize, const H: usize, const D: usize> {
    matrix: [[[Opacity<Option<A>>; W]; H]; D],
    is_active: [bool; D],
}

impl<A, const W: usize, const H: usize, const D: usize> LayeredMap<A, W, H, D> {
    fn set_active(&mut self, n: u8) {
        self.is_active[n as usize] = true
    }

    fn set_inactive(&mut self, n: u8) {
        self.is_active[n as usize] = false
    }
}

impl<A: Clone, const W: usize, const H: usize, const D: usize> Map<W, H>
    for LayeredMap<A, W, H, D>
{
    type Action = A;

    fn get(&mut self, x: usize, y: usize) -> Option<A> {
        (0..D)
            .rev()
            .filter(|z| self.is_active[*z])
            .find_map(|z| self.matrix[z][y][x].clone().into())
            .unwrap_or_default()
    }
}

#[derive(Clone, Copy, Debug)]
enum LayerAction<A> {
    Inner(A),
    SetActive(u8),
    SetInactive(u8),
}

struct LayerHandler<A, I, const W: usize, const H: usize, const D: usize>(I, PhantomData<A>);

impl<A, I, const W: usize, const H: usize, const D: usize> LayerHandler<A, I, W, H, D> {
    fn new(inner: I) -> Self {
        Self(inner, PhantomData)
    }
}

impl<
    A,
    S: RetrieveMut<LayeredMap<A, W, H, D>>,
    I: Handler<S>,
    const W: usize,
    const H: usize,
    const D: usize,
> Handler<S> for LayerHandler<A, I, W, H, D>
{
    type Action = LayerAction<I::Action>;

    fn handle(&mut self, mut state: S, action: Self::Action) {
        match action {
            LayerAction::Inner(a) => self.0.handle(state, a),
            LayerAction::SetActive(n) => state.retrieve_mut().set_inactive(n),
            LayerAction::SetInactive(n) => state.retrieve_mut().set_inactive(n),
        }
    }
}

struct DebugHandler<I>(I);

impl<I, S> Handler<S> for DebugHandler<I>
where
    I: Handler<S>,
    I::Action: core::fmt::Debug,
{
    type Action = I::Action;

    fn handle(&mut self, state: S, action: Self::Action) {
        self.0.handle(state, dbg!(action));
    }
}

#[cfg(test)]
mod tests {
    use crate::kb::builder::KeyboardBuilder;

    use super::*;

    struct MapAndInterestingState<A, const W: usize, const H: usize, const D: usize> {
        layered_map: LayeredMap<A, W, H, D>,
        interesting_state: InterestingState,
    }

    impl<A, const W: usize, const H: usize, const D: usize> Retrieve<LayeredMap<A, W, H, D>>
        for MapAndInterestingState<A, W, H, D>
    {
        fn retrieve(&self) -> &LayeredMap<A, W, H, D> {
            &self.layered_map
        }
    }

    impl<A, const W: usize, const H: usize, const D: usize> Retrieve<InterestingState>
        for MapAndInterestingState<A, W, H, D>
    {
        fn retrieve(&self) -> &InterestingState {
            &self.interesting_state
        }
    }

    impl<A, const W: usize, const H: usize, const D: usize> RetrieveMut<LayeredMap<A, W, H, D>>
        for MapAndInterestingState<A, W, H, D>
    {
        fn retrieve_mut(&mut self) -> &mut LayeredMap<A, W, H, D> {
            &mut self.layered_map
        }
    }

    impl<A, const W: usize, const H: usize, const D: usize> RetrieveMut<InterestingState>
        for MapAndInterestingState<A, W, H, D>
    {
        fn retrieve_mut(&mut self) -> &mut InterestingState {
            &mut self.interesting_state
        }
    }

    #[test]
    fn something() {
        let _keyboard = KeyboardBuilder::new()
            .scanner(DummyScanner)
            .handler(DebugHandler(LayerHandler::new(DummyHandler)))
            .state(MapAndInterestingState {
                layered_map: LayeredMap {
                    matrix: [
                        [
                            [Opacity::Opaque(Some(LayerAction::Inner(DummyAction::Foo))), Opacity::Opaque(Some(LayerAction::SetActive(1)))],
                            [Opacity::Opaque(Some(LayerAction::Inner(DummyAction::Foo))), Opacity::Opaque(Some(LayerAction::SetActive(1)))],
                        ],
                        [
                            [Opacity::Opaque(Some(LayerAction::Inner(DummyAction::Bar))), Opacity::Opaque(Some(LayerAction::SetInactive(1)))],
                            [Opacity::Opaque(Some(LayerAction::Inner(DummyAction::Bar))), Opacity::Opaque(Some(LayerAction::SetInactive(1)))],
                        ]
                    ],
                    is_active: [true, false]
                },
                interesting_state: InterestingState,
            })
            .build();
    }
}
