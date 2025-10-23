//! QMK/TMK style keycodes for ease of configuration.

use crate::action::Action;
use crate::interface::usb::KeyCode;
use crate::map::Opacity;

pub type KeyAction = Opacity<Option<Action>>;

/// Transparent.
///
/// You might want to use one of the aliases: [`KC_TRNS`], [`_______`].
pub const KC_TRANSPARENT: Opacity<Option<Action>> = Opacity::Transparent;
/// Transparent.
///
/// Alias for [`KC_TRANSPARENT`].
pub const KC_TRNS: Opacity<Option<Action>> = KC_TRANSPARENT;
/// Transparent.
///
/// Alias for [`KC_TRANSPARENT`].
pub const _______: Opacity<Option<Action>> = KC_TRANSPARENT;

/// No action.
///
/// You might want to use the alias: [`XXXXXXX`].
pub const KC_NO: Opacity<Option<Action>> = Opacity::Opaque(None);
/// No action.
///
/// Alias for [`KC_NO`].
pub const XXXXXXX: Opacity<Option<Action>> = KC_NO;

/// Activate `layer` while key is being held.
#[allow(non_snake_case)]
pub const fn MO(layer: u8) -> Opacity<Option<Action>> {
    Opacity::Opaque(Some(Action::MomentaryLayer(layer)))
}

/// Toggle active status of `layer`.
#[allow(non_snake_case)]
pub const fn TG(layer: u8) -> Opacity<Option<Action>> {
    Opacity::Opaque(Some(Action::ToggleLayer(layer)))
}

/// Registers different codes when tapped or held.
#[allow(non_snake_case)]
pub const fn TH(tapped: KeyCode, held: KeyCode) -> Opacity<Option<Action>> {
    Opacity::Opaque(Some(Action::TapHold {
        tap: tapped,
        hold: held,
    }))
}

macro_rules! define_keys {
    ($(#[doc = $doc:literal] $ident:ident $(($($alias:ident),+))? => $code:expr),* $(,)?) => {
        $(
            #[doc = $doc]
            $(
                #[doc = ""]
                #[doc = format_alias_intro!($($alias),+)]
            )?
            pub const $ident: KeyAction = Opacity::Opaque(Some(Action::Code($code)));

            $($(
                #[doc = $doc]
                #[doc = ""]
                #[doc = concat!("Alias for [`", stringify!($ident), "`].")]
                pub const $alias: KeyAction = $ident;
            )*)?
        )*
    };
}

macro_rules! format_alias_intro {
    ($alias:ident) => {
        concat!("You might want to use the alias: [`", stringify!($alias), "`].")
    };
    ($($alias:ident),+) => {
        concat!("You might want to use one of the aliases: ", format_alias_list!($($alias),+), ".")
    }
}

macro_rules! format_alias_list {
    ($alias:ident) => {
        concat!("[`", stringify!($alias), "`]")
    };
    ($first:ident, $($rest:ident),+) => {
        concat!("[`", stringify!($first), "`]", $(", [`", stringify!($rest), "`]"),+)
    };
}

define_keys! {
    /// Keyboard `a` and `A`.
    KC_A => KeyCode::KeyboardA,
    /// Keyboard `b` and `B`.
    KC_B => KeyCode::KeyboardB,
    /// Keyboard `c` and `C`.
    KC_C => KeyCode::KeyboardC,
    /// Keyboard `d` and `D`.
    KC_D => KeyCode::KeyboardD,
    /// Keyboard `e` and `E`.
    KC_E => KeyCode::KeyboardE,
    /// Keyboard `f` and `F`.
    KC_F => KeyCode::KeyboardF,
    /// Keyboard `g` and `G`.
    KC_G => KeyCode::KeyboardG,
    /// Keyboard `h` and `H`.
    KC_H => KeyCode::KeyboardH,
    /// Keyboard `i` and `I`.
    KC_I => KeyCode::KeyboardI,
    /// Keyboard `j` and `J`.
    KC_J => KeyCode::KeyboardJ,
    /// Keyboard `k` and `K`.
    KC_K => KeyCode::KeyboardK,
    /// Keyboard `l` and `L`.
    KC_L => KeyCode::KeyboardL,
    /// Keyboard `m` and `M`.
    KC_M => KeyCode::KeyboardM,
    /// Keyboard `n` and `N`.
    KC_N => KeyCode::KeyboardN,
    /// Keyboard `o` and `O`.
    KC_O => KeyCode::KeyboardO,
    /// Keyboard `p` and `P`.
    KC_P => KeyCode::KeyboardP,
    /// Keyboard `q` and `Q`.
    KC_Q => KeyCode::KeyboardQ,
    /// Keyboard `r` and `R`.
    KC_R => KeyCode::KeyboardR,
    /// Keyboard `s` and `S`.
    KC_S => KeyCode::KeyboardS,
    /// Keyboard `t` and `T`.
    KC_T => KeyCode::KeyboardT,
    /// Keyboard `u` and `U`.
    KC_U => KeyCode::KeyboardU,
    /// Keyboard `v` and `V`.
    KC_V => KeyCode::KeyboardV,
    /// Keyboard `w` and `W`.
    KC_W => KeyCode::KeyboardW,
    /// Keyboard `x` and `X`.
    KC_X => KeyCode::KeyboardX,
    /// Keyboard `y` and `Y`.
    KC_Y => KeyCode::KeyboardY,
    /// Keyboard `z` and `Z`.
    KC_Z => KeyCode::KeyboardZ,
    /// Keyboard `1` and `!`.
    KC_1 => KeyCode::Keyboard1,
    /// Keyboard `2` and `@`.
    KC_2 => KeyCode::Keyboard2,
    /// Keyboard `3` and `#`.
    KC_3 => KeyCode::Keyboard3,
    /// Keyboard `4` and `$`.
    KC_4 => KeyCode::Keyboard4,
    /// Keyboard `5` and `%`.
    KC_5 => KeyCode::Keyboard5,
    /// Keyboard `6` and `^`.
    KC_6 => KeyCode::Keyboard6,
    /// Keyboard `7` and `&`.
    KC_7 => KeyCode::Keyboard7,
    /// Keyboard `8` and `*`.
    KC_8 => KeyCode::Keyboard8,
    /// Keyboard `9` and `(`.
    KC_9 => KeyCode::Keyboard9,
    /// Keyboard `0` and `)`.
    KC_0 => KeyCode::Keyboard0,
    /// Keyboard `Enter`.
    KC_ENTER (KC_ENTR, KC_ENT) => KeyCode::Enter,
    /// Keyboard `Escape`.
    KC_ESCAPE (KC_ESC) => KeyCode::Escape,
    /// Keyboard `Backspace`.
    KC_BACKSPACE (KC_BSPC) => KeyCode::Backspace,
    /// Keyboard `Tab`.
    KC_TAB => KeyCode::Tab,
    /// Keyboard `Space`.
    KC_SPACE (KC_SPC) => KeyCode::Space,
    /// Keyboard `-` and `_`.
    KC_MINUS (KC_MINS) => KeyCode::Minus,
    /// Keyboard `=` and `+`.
    KC_EQUAL (KC_EQL) => KeyCode::Equal,
    /// Keyboard `[` and `{`.
    KC_LEFT_BRACKET (KC_LBRC) => KeyCode::LeftBracket,
    /// Keyboard `]` and `}`.
    KC_RIGHT_BRACKET (KC_RBRC) => KeyCode::RightBracket,
    /// Keyboard `\` and `|`.
    KC_BACKSLASH (KC_BSLS) => KeyCode::Backslash,
    /// Keyboard non-US `#` and `~`.
    KC_NONUS_HASH (KC_NUHS) => KeyCode::NonUSHash,
    /// Keyboard `;` and `:`.
    KC_SEMICOLON (KC_SCLN) => KeyCode::Semicolon,
    /// Keyboard `'` and `"`."#]
    KC_QUOTE (KC_QOUT) => KeyCode::Apostrophe,
    /// Keyboard `` ` `` and `~`.
    KC_GRAVE (KC_GRV) => KeyCode::Grave,
    /// Keyboard `,` and `<`.
    KC_COMMA (KC_COMM) => KeyCode::Comma,
    /// Keyboard `.` and `>`.
    KC_DOT => KeyCode::Dot,
    /// Keyboard `/` and `?`.
    KC_SLASH (KC_SLSH) => KeyCode::Slash,
    /// Keyboard `Caps Lock`.
    KC_CAPSLOCK (KC_CAPS) => KeyCode::CapsLock,
    /// Keyboard `F1`.
    KC_F1 => KeyCode::F1,
    /// Keyboard `F2`.
    KC_F2 => KeyCode::F2,
    /// Keyboard `F3`.
    KC_F3 => KeyCode::F3,
    /// Keyboard `F4`.
    KC_F4 => KeyCode::F4,
    /// Keyboard `F5`.
    KC_F5 => KeyCode::F5,
    /// Keyboard `F6`.
    KC_F6 => KeyCode::F6,
    /// Keyboard `F7`.
    KC_F7 => KeyCode::F7,
    /// Keyboard `F8`.
    KC_F8 => KeyCode::F8,
    /// Keyboard `F9`.
    KC_F9 => KeyCode::F9,
    /// Keyboard `F10`.
    KC_F10 => KeyCode::F10,
    /// Keyboard `F11`.
    KC_F11 => KeyCode::F11,
    /// Keyboard `F12`.
    KC_F12 => KeyCode::F12,
    /// Keyboard `Print Screen`.
    KC_PRINT_SCREEN (KC_PSCR) => KeyCode::PrintScreen,
    /// Keyboard `Scroll Lock`.
    KC_SCROLL_LOCK (KC_SCRL) => KeyCode::ScrollLock,
    /// Keyboard `Pause`.
    KC_PAUSE (KC_PAUS) => KeyCode::Pause,
    /// Keyboard `Insert`.
    KC_INSERT (KC_INS) => KeyCode::Insert,
    /// Keyboard `Home`.
    KC_HOME => KeyCode::Home,
    /// Keyboard `Page Up`.
    KC_PAGE_UP (KC_PGUP) => KeyCode::PageUp,
    /// Keyboard `Delete`.
    KC_DELETE (KC_DEL) => KeyCode::Delete,
    /// Keyboard `End`.
    KC_END => KeyCode::End,
    /// Keyboard `Page Down`.
    KC_PAGE_DOWN (KC_PGDN) => KeyCode::PageDown,
    /// Keyboard `Right Arrow`.
    KC_RIGHT (KC_RGHT) => KeyCode::RightArrow,
    /// Keyboard `Left Arrow`.
    KC_LEFT => KeyCode::LeftArrow,
    /// Keyboard `Down Arrow`.
    KC_DOWN => KeyCode::DownArrow,
    /// Keyboard `Up Arrow`.
    KC_UP => KeyCode::UpArrow,
    /// Keypad `Num Lock`.
    KC_NUM_LOCK (KC_NUM) => KeyCode::KeypadNumLock,
    /// Keypad `/`.
    KC_KP_DIVIDE => KeyCode::KeypadDivide,
    /// Keypad `*`.
    KC_KP_MULTIPLY => KeyCode::KeypadMultiply,
    /// Keypad `-`.
    KC_KP_MINUS => KeyCode::KeypadSubtract,
    /// Keypad `+`.
    KC_KP_PLUS => KeyCode::KeypadAdd,
    /// Keypad `Enter`.
    KC_KP_ENTER => KeyCode::KeypadEnter,
    /// Keypad `1` and `End`.
    KC_KP_1 => KeyCode::Keypad1,
    /// Keypad `2` and `Down Arrow`.
    KC_KP_2 => KeyCode::Keypad2,
    /// Keypad `3` and `Page Down`.
    KC_KP_3 => KeyCode::Keypad3,
    /// Keypad `4` and `Left Arrow`.
    KC_KP_4 => KeyCode::Keypad4,
    /// Keypad `5`.
    KC_KP_5 => KeyCode::Keypad5,
    /// Keypad `6` and `Right Arrow`.
    KC_KP_6 => KeyCode::Keypad6,
    /// Keypad `7` and `Home`.
    KC_KP_7 => KeyCode::Keypad7,
    /// Keypad `8` and `Up Arrow`.
    KC_KP_8 => KeyCode::Keypad8,
    /// Keypad `9` and `Page Up`.
    KC_KP_9 => KeyCode::Keypad9,
    /// Keypad `0` and `Insert`.
    KC_KP_0 => KeyCode::Keypad0,
    /// Keypad `.` and `Delete`.
    KC_KP_DOT => KeyCode::KeypadDot,
    /// Keyboard non-US `\` and `|`.
    KC_NONUS_BACKSLASH (KC_NUBS) => KeyCode::NonUSBackslash,
    /// Keyboard `Application`.
    KC_APPLICATION (KC_APP) => KeyCode::Application,
    /// Keyboard `Power`.
    KC_POWER => KeyCode::Power,
    /// Keyboard `=`.
    KC_KP_EQUAL => KeyCode::KeypadEqual,
    /// Keyboard `F13`.
    KC_F13 => KeyCode::F13,
    /// Keyboard `F14`.
    KC_F14 => KeyCode::F14,
    /// Keyboard `F15`.
    KC_F15 => KeyCode::F15,
    /// Keyboard `F16`.
    KC_F16 => KeyCode::F16,
    /// Keyboard `F17`.
    KC_F17 => KeyCode::F17,
    /// Keyboard `F18`.
    KC_F18 => KeyCode::F18,
    /// Keyboard `F19`.
    KC_F19 => KeyCode::F19,
    /// Keyboard `F20`.
    KC_F20 => KeyCode::F20,
    /// Keyboard `F21`.
    KC_F21 => KeyCode::F21,
    /// Keyboard `F22`.
    KC_F22 => KeyCode::F22,
    /// Keyboard `F23`.
    KC_F23 => KeyCode::F23,
    /// Keyboard `F24`.
    KC_F24 => KeyCode::F24,
    /// Keyboard `Execute`.
    KC_EXECUTE (KC_EXEC) => KeyCode::Execute,
    /// Keyboard `Help`.
    KC_HELP => KeyCode::Help,
    /// Keyboard `Menu`.
    KC_MENU => KeyCode::Menu,
    /// Keyboard `Select`.
    KC_SELECT (KC_SLCT) => KeyCode::Select,
    /// Keyboard `Stop`.
    KC_STOP => KeyCode::Stop,
    /// Keyboard `Again`.
    KC_AGAIN (KC_AGIN) => KeyCode::Again,
    /// Keyboard `Undo`.
    KC_UNDO => KeyCode::Undo,
    /// Keyboard `Cut`.
    KC_CUT => KeyCode::Cut,
    /// Keyboard `Copy`.
    KC_COPY => KeyCode::Copy,
    /// Keyboard `Paste`.
    KC_PASTE (KC_PSTE) => KeyCode::Paste,
    /// Keyboard `Find`.
    KC_FIND => KeyCode::Find,
    /// Keyboard `Mute`.
    KC_MUTE => KeyCode::Mute,
    /// Keyboard `Volume Up`.
    KC_VOLUME_UP (KC_VLUP) => KeyCode::VolumeUp,
    /// Keyboard `Volume Down`.
    KC_VOLUME_DOWN (KC_VLDN) => KeyCode::VolumeDown,
    /// Keyboard `Locking Caps Lock`.
    KC_LOCKING_CAPS_LOCK (KC_LCAP) => KeyCode::LockingCapsLock,
    /// Keyboard `Locking Num Lock`.
    KC_LOCKING_NUM (KC_LNUM) => KeyCode::LockingNumLock,
    /// Keyboard `Locking Scroll Lock`.
    KC_LOCKING_SCROLL (KC_LSCR)=> KeyCode::LockingScrollLock,
    /// Keypad `,`.
    KC_KP_COMMA => KeyCode::KeypadComma,
    /// Keypad `=`.
    KC_KP_EQUAL_AS400 => KeyCode::KeypadEqualSign,
    /// Keyboard `International 1`.
    KC_INTERNATIONAL_1 (KC_INT1) => KeyCode::International1,
    /// Keyboard `International 2`.
    KC_INTERNATIONAL_2 (KC_INT2) => KeyCode::International2,
    /// Keyboard `International 3`.
    KC_INTERNATIONAL_3 (KC_INT3) => KeyCode::International3,
    /// Keyboard `International 4`.
    KC_INTERNATIONAL_4 (KC_INT4) => KeyCode::International4,
    /// Keyboard `International 5`.
    KC_INTERNATIONAL_5 (KC_INT5) => KeyCode::International5,
    /// Keyboard `International 6`.
    KC_INTERNATIONAL_6 (KC_INT6) => KeyCode::International6,
    /// Keyboard `International 7`.
    KC_INTERNATIONAL_7 (KC_INT7) => KeyCode::International7,
    /// Keyboard `International 8`.
    KC_INTERNATIONAL_8 (KC_INT8) => KeyCode::International8,
    /// Keyboard `International 9`.
    KC_INTERNATIONAL_9 (KC_INT9) => KeyCode::International9,
    /// Keyboard `Language 1`.
    KC_LANGUAGE_1 (KC_LANG1) => KeyCode::Language1,
    /// Keyboard `Language 2`.
    KC_LANGUAGE_2 (KC_LANG2) => KeyCode::Language2,
    /// Keyboard `Language 3`.
    KC_LANGUAGE_3 (KC_LANG3) => KeyCode::Language3,
    /// Keyboard `Language 4`.
    KC_LANGUAGE_4 (KC_LANG4) => KeyCode::Language4,
    /// Keyboard `Language 5`.
    KC_LANGUAGE_5 (KC_LANG5) => KeyCode::Language5,
    /// Keyboard `Language 6`.
    KC_LANGUAGE_6 (KC_LANG6) => KeyCode::Language6,
    /// Keyboard `Language 7`.
    KC_LANGUAGE_7 (KC_LANG7) => KeyCode::Language7,
    /// Keyboard `Language 8`.
    KC_LANGUAGE_8 (KC_LANG8) => KeyCode::Language8,
    /// Keyboard `Language 9`.
    KC_LANGUAGE_9 (KC_LANG9) => KeyCode::Language9,
    /// Keyboard `Alternate Erase`.
    KC_ALTERNATE_ERASE (KC_ERAS) => KeyCode::AlternateErase,
    /// Keyboard `SysReq/Attention`.
    KC_SYSREQ => KeyCode::SysReqAttention,
    /// Keyboard `Cancel`.
    KC_CANCEL (KC_CNCL) => KeyCode::Cancel,
    /// Keyboard `Clear`.
    KC_CLEAR (KC_CRL) => KeyCode::Clear,
    /// Keyboard `Prior`.
    KC_PRIOR (KC_PRIR) => KeyCode::Prior,
    /// Keyboard `Return`.
    KC_RETURN (KC_RETN, KC_RET) => KeyCode::Return,
    /// Keyboard `Separator`.
    KC_SEPARATOR (KC_SEPR) => KeyCode::Separator,
    /// Keyboard `Out`.
    KC_OUT => KeyCode::Out,
    /// Keyboard `Oper`.
    KC_OPER => KeyCode::Oper,
    /// Keyboard `Clear/Again`.
    KC_CLEAR_AGAIN (KC_CLAG) => KeyCode::ClearAgain,
    /// Keyboard `CrSel/Props`.
    KC_CRSEL (KC_CRSL) => KeyCode::CrSelProps,
    /// Keyboard `ExSel`.
    KC_EXSEL (KC_EXSL) => KeyCode::ExSel,
    /// Keyboard `Left Control`.
    KC_LEFT_CONTROL (KC_LCTL) => KeyCode::LeftControl,
    /// Keyboard `Left Shift`.
    KC_LEFT_SHIFT (KC_LSFT) => KeyCode::LeftShift,
    /// Keyboard `Left ALT`.
    KC_LEFT_ALT (KC_LALT) => KeyCode::LeftAlt,
    /// Keyboard `Left GUI`.
    KC_LEFT_GUI (KC_LGUI, KC_LCMD, KC_LWIN) => KeyCode::LeftGUI,
    /// Keyboard `Right Control`.
    KC_RIGHT_CONTROL (KC_RCTL) => KeyCode::RightControl,
    /// Keyboard `Right Shift`.
    KC_RIGHT_SHIFT (KC_RSFT) => KeyCode::RightShift,
    /// Keyboard `Right ALT`.
    KC_RIGHT_ALT (KC_RALT) => KeyCode::RightAlt,
    /// Keyboard `Right GUI`.
    KC_RIGHT_GUI (KC_RGUI, KC_RCMD, KC_RWIN) => KeyCode::RightGUI,
}
