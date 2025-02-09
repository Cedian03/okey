mod key;
mod modifier;

pub use key::Key;
pub use modifier::Modifier;

#[derive(Clone, Copy, Debug, Default)]
pub enum Action {
    #[default]
    NoAction,
    Key(Key),
    Modifier(Modifier),
    MomentaryLayer(u8),
    ToggleLayer(u8),
}