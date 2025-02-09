mod key;
mod modifier;

pub use key::Key;
pub use modifier::Modifier;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Key(Key),
    Modifier(Modifier),
    MomentaryLayer(u8),
    ToggleLayer(u8),
}