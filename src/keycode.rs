#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum KeyCode {
    /// No event indicated.
    NoEvent = 0x00,
    /// Roll-over error.
    RollOverError = 0x01,
    // POSTFail = 0x02, // TODO
    /// Undefined error.
    UndefinedError = 0x03,
    /// Keyboard `a` and `A`.
    KeyboardA = 0x04,
    /// Keyboard `b` and `B`.
    KeyboardB = 0x05,
    /// Keyboard `c` and `C`.
    KeyboardC = 0x06,
    /// Keyboard `d` and `D`.
    KeyboardD = 0x07,
    /// Keyboard `e` and `E`.
    KeyboardE = 0x08,
    /// Keyboard `f` and `F`.
    KeyboardF = 0x09,
    /// Keyboard `g` and `G`.
    KeyboardG = 0x0A,
    /// Keyboard `h` and `H`.
    KeyboardH = 0x0B,
    /// Keyboard `i` and `I`.
    KeyboardI = 0x0C,
    /// Keyboard `j` and `J`.
    KeyboardJ = 0x0D,
    /// Keyboard `k` and `K`.
    KeyboardK = 0x0E,
    /// Keyboard `l` and `L`.
    KeyboardL = 0x0F,
    /// Keyboard `m` and `M`.
    KeyboardM = 0x10,
    /// Keyboard `n` and `N`.
    KeyboardN = 0x11,
    /// Keyboard `o` and `O`.
    KeyboardO = 0x12,
    /// Keyboard `p` and `P`.
    KeyboardP = 0x13,
    /// Keyboard `q` and `Q`.
    KeyboardQ = 0x14,
    /// Keyboard `r` and `R`.
    KeyboardR = 0x15,
    /// Keyboard `s` and `S`.
    KeyboardS = 0x16,
    /// Keyboard `t` and `T`.
    KeyboardT = 0x17,
    /// Keyboard `u` and `U`.
    KeyboardU = 0x18,
    /// Keyboard `v` and `V`.
    KeyboardV = 0x19,
    /// Keyboard `w` and `W`.
    KeyboardW = 0x1A,
    /// Keyboard `x` and `X`.
    KeyboardX = 0x1B,
    /// Keyboard `y` and `Y`.
    KeyboardY = 0x1C,
    /// Keyboard `z` and `Z`.
    KeyboardZ = 0x1D,
    /// Keyboard `1` and `!`.
    Keyboard1 = 0x1E,
    /// Keyboard `2` and `@`.
    Keyboard2 = 0x1F,
    /// Keyboard `3` and `#`.
    Keyboard3 = 0x20,
    /// Keyboard `4` and `$`.
    Keyboard4 = 0x21,
    /// Keyboard `5` and `%`.
    Keyboard5 = 0x22,
    /// Keyboard `6` and `^`.
    Keyboard6 = 0x23,
    /// Keyboard `7` and `&`.
    Keyboard7 = 0x24,
    /// Keyboard `8` and `*`.
    Keyboard8 = 0x25,
    /// Keyboard `9` and `(`.
    Keyboard9 = 0x26,
    /// Keyboard `0` and `)`.
    Keyboard0 = 0x27,
    /// Keyboard `Enter`.
    Enter = 0x28,
    /// Keyboard `Escape`.
    Escape = 0x29,
    /// Keyboard `Backspace`.
    Backspace = 0x2A,
    /// Keyboard `Tab`.
    Tab = 0x2B,
    /// Keyboard `Space`.
    Space = 0x2C,
    /// Keyboard `-` and `_`.
    Minus = 0x2D,
    /// Keyboard `=` and `+`.
    Equal = 0x2E,
    /// Keyboard `[` and `{`.
    LeftBracket = 0x2F,
    /// Keyboard `]` and `}`.
    RightBracket = 0x30,
    /// Keyboard `\` and `|`.
    Backslash = 0x31,
    /// Keyboard non-US `#` and `~`.
    NonUSHash = 0x32,
    /// Keyboard `;` and `:`.
    Semicolon = 0x33,
    /// Keyboard `'` and `"`.
    Apostrophe = 0x34,
    /// Keyboard `` ` `` and `~`.
    Grave = 0x35,
    /// Keyboard `,` and `<`.
    Comma = 0x36,
    /// Keyboard `.` and `>`.
    Dot = 0x37,
    /// Keyboard `/` and `?`.
    Slash = 0x38,
    /// Keyboard `Caps Lock`.
    CapsLock = 0x39,
    /// Keyboard `F1`.
    F1 = 0x3A,
    /// Keyboard `F2`.
    F2 = 0x3B,
    /// Keyboard `F3`.
    F3 = 0x3C,
    /// Keyboard `F4`.
    F4 = 0x3D,
    /// Keyboard `F5`.
    F5 = 0x3E,
    /// Keyboard `F6`.
    F6 = 0x3F,
    /// Keyboard `F7`.
    F7 = 0x40,
    /// Keyboard `F8`.
    F8 = 0x41,
    /// Keyboard `F9`.
    F9 = 0x42,
    /// Keyboard `F10`.
    F10 = 0x43,
    /// Keyboard `F11`.
    F11 = 0x44,
    /// Keyboard `F12`.
    F12 = 0x45,
    /// Keyboard `Print Screen`.
    PrintScreen = 0x46,
    /// Keyboard `Scroll Lock`.
    ScrollLock = 0x47,
    /// Keyboard `Pause`.
    Pause = 0x48,
    /// Keyboard `Insert`.
    Insert = 0x49,
    /// Keyboard `Home`.
    Home = 0x4A,
    /// Keyboard `Page Up`.
    PageUp = 0x4B,
    /// Keyboard `Delete`.
    Delete = 0x4C,
    /// Keyboard `End`.
    End = 0x4D,
    /// Keyboard `Page Down`.
    PageDown = 0x4E,
    /// Keyboard `Right Arrow`.
    RightArrow = 0x4F,
    /// Keyboard `Left Arrow`.
    LeftArrow = 0x50,
    /// Keyboard `Down Arrow`.
    DownArrow = 0x51,
    /// Keyboard `Up Arrow`.
    UpArrow = 0x52,
    /// Keypad `Num Lock`.
    KeypadNumLock = 0x53,
    /// Keypad `/`.
    KeypadDivide = 0x54,
    /// Keypad `*`.
    KeypadMultiply = 0x55,
    /// Keypad `-`.
    KeypadSubtract = 0x56,
    /// Keypad `+`.
    KeypadAdd = 0x57,
    /// Keypad `Enter`.
    KeypadEnter = 0x58,
    /// Keypad `1` and `End`.
    Keypad1 = 0x59,
    /// Keypad `2` and `Down Arrow`.
    Keypad2 = 0x5A,
    /// Keypad `3` and `Page Down`.
    Keypad3 = 0x5B,
    /// Keypad `4` and `Left Arrow`.
    Keypad4 = 0x5C,
    /// Keypad `5`.
    Keypad5 = 0x5D,
    /// Keypad `6` and `Right Arrow`.
    Keypad6 = 0x5E,
    /// Keypad `7` and `Home`.
    Keypad7 = 0x5F,
    /// Keypad `8` and `Up Arrow`.
    Keypad8 = 0x60,
    /// Keypad `9` and `Page Up`.
    Keypad9 = 0x61,
    /// Keypad `0` and `Insert`.
    Keypad0 = 0x62,
    /// Keypad `.` and `Delete`.
    KeypadDot = 0x63,
    /// Keyboard non-US `\` and `|`.
    NonUSBackslash = 0x64,
    /// Keyboard `Application`.
    Application = 0x65,
    /// Keyboard `Power`.
    Power = 0x66,
    /// Keyboard `=`.
    KeypadEqual = 0x67,
    /// Keyboard `F13`.
    F13 = 0x68,
    /// Keyboard `F14`.
    F14 = 0x69,
    /// Keyboard `F15`.
    F15 = 0x6A,
    /// Keyboard `F16`.
    F16 = 0x6B,
    /// Keyboard `F17`.
    F17 = 0x6C,
    /// Keyboard `F18`.
    F18 = 0x6D,
    /// Keyboard `F19`.
    F19 = 0x6E,
    /// Keyboard `F20`.
    F20 = 0x6F,
    /// Keyboard `F21`.
    F21 = 0x70,
    /// Keyboard `F22`.
    F22 = 0x71,
    /// Keyboard `F23`.
    F23 = 0x72,
    /// Keyboard `F24`.
    F24 = 0x73,
    /// Keyboard `Execute`.
    Execute = 0x74,
    /// Keyboard `Help`.
    Help = 0x75,
    /// Keyboard `Menu`.
    Menu = 0x76,
    /// Keyboard `Select`.
    Select = 0x77,
    /// Keyboard `Stop`.
    Stop = 0x78,
    /// Keyboard `Again`.
    Again = 0x79,
    /// Keyboard `Undo`.
    Undo = 0x7A,
    /// Keyboard `Cut`.
    Cut = 0x7B,
    /// Keyboard `Copy`.
    Copy = 0x7C,
    /// Keyboard `Paste`.
    Paste = 0x7D,
    /// Keyboard `Find`.
    Find = 0x7E,
    /// Keyboard `Mute`.
    Mute = 0x7F,
    /// Keyboard `Volume Up`.
    VolumeUp = 0x80,
    /// Keyboard `Volume Down`.
    VolumeDown = 0x81,
    /// Keyboard `Locking Caps Lock`.
    LockingCapsLock = 0x82,
    /// Keyboard `Locking Num Lock`.
    LockingNumLock = 0x83,
    /// Keyboard `Locking Scroll Lock`.
    LockingScrollLock = 0x84,
    /// Keypad `,`.
    KeypadComma = 0x85,
    /// Keypad `=`.
    KeypadEqualSign = 0x86,
    /// Keyboard `International 1`.
    International1 = 0x87,
    /// Keyboard `International 2`.
    International2 = 0x88,
    /// Keyboard `International 3`.
    International3 = 0x89,
    /// Keyboard `International 4`.
    International4 = 0x8A,
    /// Keyboard `International 5`.
    International5 = 0x8B,
    /// Keyboard `International 6`.
    International6 = 0x8C,
    /// Keyboard `International 7`.
    International7 = 0x8D,
    /// Keyboard `International 8`.
    International8 = 0x8E,
    /// Keyboard `International 9`.
    International9 = 0x8F,
    /// Keyboard `Language 1`.
    Language1 = 0x90,
    /// Keyboard `Language 2`.
    Language2 = 0x91,
    /// Keyboard `Language 3`.
    Language3 = 0x92,
    /// Keyboard `Language 4`.
    Language4 = 0x93,
    /// Keyboard `Language 5`.
    Language5 = 0x94,
    /// Keyboard `Language 6`.
    Language6 = 0x95,
    /// Keyboard `Language 7`.
    Language7 = 0x96,
    /// Keyboard `Language 8`.
    Language8 = 0x97,
    /// Keyboard `Language 9`.
    Language9 = 0x98,
    /// Keyboard `Alternate Erase`. 
    AlternateErase = 0x99,
    /// Keyboard `SysReq/Attention`. 
    SysReqAttention = 0x9A,
    /// Keyboard `Cancel`. 
    Cancel = 0x9B,
    /// Keyboard `Clear`. 
    Clear = 0x9C,
    /// Keyboard `Prior`. 
    Prior = 0x9D,
    /// Keyboard `Return`. 
    Return = 0x9E,
    /// Keyboard `Separator`. 
    Separator = 0x9F,
    /// Keyboard `Out`. 
    Out = 0xA0,
    /// Keyboard `Oper`. 
    Oper = 0xA1,
    /// Keyboard `Clear/Again`. 
    ClearAgain = 0xA2,
    /// Keyboard `CrSel/Props`. 
    CrSelProps = 0xA3,
    /// Keyboard `ExSel`. 
    ExSel = 0xA4,

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

impl KeyCode {
    pub fn is_error(self) -> bool {
        (Self::RollOverError..=Self::UndefinedError).contains(&self)
    }

    pub fn is_modifier(self) -> bool {
        (Self::LeftControl..=Self::RightGUI).contains(&self)
    }

    pub fn is_any(self) -> bool {
        self != Self::NoEvent
    }

    pub fn modifier_index(self) -> Option<u8> {
        self.is_modifier().then(|| u8::from(self) & 0x07)        
    }

    pub fn modifier_mask(self) -> Option<u8> {
        self.modifier_index().map(|x| 1 << x)
    }
}

impl From<KeyCode> for u8 {
    fn from(value: KeyCode) -> Self {
        value as u8
    }
}