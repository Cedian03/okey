#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    Pressed,
    Released,
    Held,
}