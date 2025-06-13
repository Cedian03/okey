use super::*;

pub struct KeyboardBuilder<const W: usize, const H: usize, Scanner = (), _Handler = (), _Map = ()> {
    scanner: Scanner,
    handler: _Handler,
    state: _Map,
}

impl<const W: usize, const H: usize> KeyboardBuilder<W, H> {
    pub fn new() -> KeyboardBuilder<W, H> {
        KeyboardBuilder {
            scanner: (),
            handler: (),
            state: (),
        }
    }

    pub fn scanner<S>(self, scanner: S) -> KeyboardBuilder<W, H, S> {
        KeyboardBuilder {
            scanner,
            handler: (),
            state: (),
        }
    }
}

impl<S: Scan<W, H>, const W: usize, const H: usize> KeyboardBuilder<W, H, S, (), ()> {
    pub fn handler<AH>(self, handler: AH) -> KeyboardBuilder<W, H, S, AH> {
        KeyboardBuilder {
            scanner: self.scanner,
            handler,
            state: (),
        }
    }
}

impl<SCANNER, HANDLER, const W: usize, const H: usize> KeyboardBuilder<W, H, SCANNER, HANDLER, ()> {
    pub fn state<STATE>(self, state: STATE) -> KeyboardBuilder<W, H, SCANNER, HANDLER, STATE> {
        KeyboardBuilder {
            scanner: self.scanner,
            handler: self.handler,
            state,
        }
    }
}

impl<SCANNER, HANDLER, STATE, const W: usize, const H: usize>
    KeyboardBuilder<W, H, SCANNER, HANDLER, STATE>
{
    pub fn build<MAP>(self) -> Keyboard<SCANNER, HANDLER, STATE, MAP, W, H>
    where
        SCANNER: Scan<W, H>,
        HANDLER: Handler<STATE>,
        STATE: Retrieve<MAP>,
        MAP: Map<W, H, Action = HANDLER::Action>,
    {
        Keyboard::new(self.scanner, self.handler, self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_without_type() {
        let mut _keyboard = KeyboardBuilder::new()
            .scanner(DummyScanner)
            .handler(DummyHandler)
            .state(MatrixMap([
                [DummyAction::Foo, DummyAction::Bar],
                [DummyAction::Foo, DummyAction::Bar],
            ]))
            .build();
    }
}
