#![allow(dead_code)]

struct Board<T: EventHandler> {
    event_handler: T,
}

trait EventHandler {
    fn handle_event(&mut self, x: usize, y: usize, event: Event);
}

#[derive(Clone, Copy, Debug)]
struct Event {
    position: Position,
    kind: EventKind,
}

#[derive(Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EventKind {
    Press,
    Release,
}

trait ActionHandler {
    type Action;

    fn handle_action(&mut self, action: Self::Action, event: Event);
}

struct USB;

#[derive(Copy, Clone, Debug)]
enum USBAction {
    A,
    B,
    C,
    // ...
}

impl ActionHandler for USB {
    type Action = USBAction;

    fn handle_action(&mut self, action: Self::Action, _event: Event) {
        println!("[USB] handling {:?}...", action)
    }
}

struct PS2;

#[derive(Copy, Clone, Debug)]
enum PS2Action {
    X,
    Y,
    Z,
    // ...
}

impl ActionHandler for PS2 {
    type Action = PS2Action;

    fn handle_action(&mut self, action: Self::Action, _event: Event) {
        println!("[PS2] handling {:?}...", action)
    }
}

struct LayeredActionMap<T: ActionHandler, const W: usize, const H: usize, const D: usize> {
    layers: [[[LayeredActionCell<T::Action>; W]; H]; D],
    layer_active: [bool; D],
    inner: T,
}

#[derive(Clone, Debug)]
enum LayeredActionCell<T> {
    Opaque(LayeredAction<T>),
    Transparent,
}

#[derive(Clone, Debug)]
enum LayeredAction<T> {
    Inner(T),
    Momentary(u8),
    Toggle(u8),
}

impl<T: ActionHandler, const W: usize, const H: usize, const D: usize>
    LayeredActionMap<T, W, H, D>
{
    fn get(&self, x: usize, y: usize) -> Option<LayeredAction<T::Action>>
    where
        T::Action: Clone,
    {
        for z in (0..self.layer_active.len()).rev() {
            if self.layer_active[z] {
                if let LayeredActionCell::Opaque(a) = self.layers[z][y][x].clone() {
                    return Some(a);
                }
            }
        }

        None
    }
}

impl<T: ActionHandler, const W: usize, const H: usize, const D: usize> EventHandler
    for LayeredActionMap<T, W, H, D>
where
    T::Action: Clone,
{
    fn handle_event(&mut self, x: usize, y: usize, event: Event) {
        if let Some(action) = self.get(x, y) {
            self.handle_action(action, event);
        }
    }
}

impl<T: ActionHandler, const W: usize, const H: usize, const D: usize> ActionHandler
    for LayeredActionMap<T, W, H, D>
{
    type Action = LayeredAction<T::Action>;

    fn handle_action(&mut self, action: Self::Action, event: Event) {
        match action {
            LayeredAction::Inner(t) => self.inner.handle_action(t, event),
            LayeredAction::Momentary(i) => {
                self.layer_active[i as usize] = event.kind == EventKind::Press
            }
            LayeredAction::Toggle(i) => {
                if event.kind == EventKind::Press {
                    self.layer_active[i as usize] ^= true
                }
            }
        }
    }
}

struct FooHandler<T> {
    inner: T,
}

#[derive(Clone, Debug)]
enum FooAction<T> {
    Inner(T),
    Foo,
}

impl<T: ActionHandler> ActionHandler for FooHandler<T> {
    type Action = FooAction<T::Action>;

    fn handle_action(&mut self, action: Self::Action, event: Event) {
        match action {
            FooAction::Inner(a) => self.inner.handle_action(a, event),
            FooAction::Foo => println!("[FOO] foobar!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let _board = Board {
            event_handler: LayeredActionMap {
                layers: [
                    [
                        [
                            LayeredActionCell::Opaque(LayeredAction::Inner(FooAction::Inner(
                                USBAction::A,
                            ))),
                            LayeredActionCell::Opaque(LayeredAction::Inner(FooAction::Inner(
                                USBAction::B,
                            ))),
                        ],
                        [
                            LayeredActionCell::Opaque(LayeredAction::Inner(FooAction::Inner(
                                USBAction::C,
                            ))),
                            LayeredActionCell::Transparent,
                        ],
                    ],
                    [
                        [
                            LayeredActionCell::Opaque(LayeredAction::Toggle(0)),
                            LayeredActionCell::Opaque(LayeredAction::Toggle(1)),
                        ],
                        [
                            LayeredActionCell::Transparent,
                            LayeredActionCell::Opaque(LayeredAction::Inner(FooAction::Inner(
                                USBAction::A,
                            ))),
                        ],
                    ],
                ],
                layer_active: [true, true],
                inner: FooHandler { inner: USB },
            },
        };
    }
}
