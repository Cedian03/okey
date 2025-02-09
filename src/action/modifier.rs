#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Modifier {
    /// Keyboard `Left Control`.
    LeftControl = 0xE0,
    /// Keyboard `Left Shift`.
    LeftShift = 0xE1,
    /// Keyboard `Left Alt`.
    LeftAlt = 0xE2,
    /// Keyboard `Left GUI`.
    LeftGUI = 0xE3,
    /// Keyboard `Right Control`.
    RightControl = 0xE4,
    /// Keyboard `Right Shift`.
    RightShift = 0xE5,
    /// Keyboard `Right Alt`.
    RightAlt = 0xE6,
    /// Keyboard `Right GUI`.
    RightGUI = 0xE7,
}

impl Modifier {
    pub fn mask(self) -> u8 {
        1 << self.index()
    }

    pub fn index(self) -> u8 {
        self as u8 & 0x07
    }
}