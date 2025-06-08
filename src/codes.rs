//! QMK/TMK style keycodes for ease of configuration.

use crate::action::Action;
use crate::action_map::Opacity;
use crate::interface::usb::Code;

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
pub const fn TH(tapped: Code, held: Code) -> Opacity<Option<Action>> {
    Opacity::Opaque(Some(Action::TapHold(tapped, held)))
}

macro_rules! define_keys {
    ($(#[doc = $doc:literal] $ident:ident $(($($alias:ident),+))? => $code:expr),* $(,)?) => {
        $(
            #[doc = $doc]
            $(
                #[doc = ""]
                #[doc = format_alias_intro!($($alias),+)]
            )?
            pub const $ident: Opacity<Option<Action>> = Opacity::Opaque(Some(Action::Code($code)));

            $($(
                #[doc = $doc]
                #[doc = ""]
                #[doc = concat!("Alias for [`", stringify!($ident), "`].")]
                pub const $alias: Opacity<Option<Action>> = $ident;
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
    KC_A => Code::KeyboardA,
    /// Keyboard `b` and `B`.
    KC_B => Code::KeyboardB,
    /// Keyboard `c` and `C`.
    KC_C => Code::KeyboardC,
    /// Keyboard `d` and `D`.
    KC_D => Code::KeyboardD,
    /// Keyboard `e` and `E`.
    KC_E => Code::KeyboardE,
    /// Keyboard `f` and `F`.
    KC_F => Code::KeyboardF,
    /// Keyboard `g` and `G`.
    KC_G => Code::KeyboardG,
    /// Keyboard `h` and `H`.
    KC_H => Code::KeyboardH,
    /// Keyboard `i` and `I`.
    KC_I => Code::KeyboardI,
    /// Keyboard `j` and `J`.
    KC_J => Code::KeyboardJ,
    /// Keyboard `k` and `K`.
    KC_K => Code::KeyboardK,
    /// Keyboard `l` and `L`.
    KC_L => Code::KeyboardL,
    /// Keyboard `m` and `M`.
    KC_M => Code::KeyboardM,
    /// Keyboard `n` and `N`.
    KC_N => Code::KeyboardN,
    /// Keyboard `o` and `O`.
    KC_O => Code::KeyboardO,
    /// Keyboard `p` and `P`.
    KC_P => Code::KeyboardP,
    /// Keyboard `q` and `Q`.
    KC_Q => Code::KeyboardQ,
    /// Keyboard `r` and `R`.
    KC_R => Code::KeyboardR,
    /// Keyboard `s` and `S`.
    KC_S => Code::KeyboardS,
    /// Keyboard `t` and `T`.
    KC_T => Code::KeyboardT,
    /// Keyboard `u` and `U`.
    KC_U => Code::KeyboardU,
    /// Keyboard `v` and `V`.
    KC_V => Code::KeyboardV,
    /// Keyboard `w` and `W`.
    KC_W => Code::KeyboardW,
    /// Keyboard `x` and `X`.
    KC_X => Code::KeyboardX,
    /// Keyboard `y` and `Y`.
    KC_Y => Code::KeyboardY,
    /// Keyboard `z` and `Z`.
    KC_Z => Code::KeyboardZ,
    /// Keyboard `1` and `!`.
    KC_1 => Code::Keyboard1,
    /// Keyboard `2` and `@`.
    KC_2 => Code::Keyboard2,
    /// Keyboard `3` and `#`.
    KC_3 => Code::Keyboard3,
    /// Keyboard `4` and `$`.
    KC_4 => Code::Keyboard4,
    /// Keyboard `5` and `%`.
    KC_5 => Code::Keyboard5,
    /// Keyboard `6` and `^`.
    KC_6 => Code::Keyboard6,
    /// Keyboard `7` and `&`.
    KC_7 => Code::Keyboard7,
    /// Keyboard `8` and `*`.
    KC_8 => Code::Keyboard8,
    /// Keyboard `9` and `(`.
    KC_9 => Code::Keyboard9,
    /// Keyboard `0` and `)`.
    KC_0 => Code::Keyboard0,
    /// Keyboard `Enter`.
    KC_ENTER (KC_ENTR, KC_ENT) => Code::Enter,
    /// Keyboard `Escape`.
    KC_ESCAPE (KC_ESC) => Code::Escape,
    /// Keyboard `Backspace`.
    KC_BACKSPACE (KC_BSPC) => Code::Backspace,
    /// Keyboard `Tab`.
    KC_TAB => Code::Tab,
    /// Keyboard `Space`.
    KC_SPACE (KC_SPC) => Code::Space,
    /// Keyboard `-` and `_`.
    KC_MINUS (KC_MINS) => Code::Minus,
    /// Keyboard `=` and `+`.
    KC_EQUAL (KC_EQL) => Code::Equal,
    /// Keyboard `[` and `{`.
    KC_LEFT_BRACKET (KC_LBRC) => Code::LeftBracket,
    /// Keyboard `]` and `}`.
    KC_RIGHT_BRACKET (KC_RBRC) => Code::RightBracket,
    /// Keyboard `\` and `|`.
    KC_BACKSLASH (KC_BSLS) => Code::Backslash,
    /// Keyboard non-US `#` and `~`.
    KC_NONUS_HASH (KC_NUHS) => Code::NonUSHash,
    /// Keyboard `;` and `:`.
    KC_SEMICOLON (KC_SCLN) => Code::Semicolon,
    /// Keyboard `'` and `"`."#]
    KC_QUOTE (KC_QOUT) => Code::Apostrophe,
    /// Keyboard `` ` `` and `~`.
    KC_GRAVE (KC_GRV) => Code::Grave,
    /// Keyboard `,` and `<`.
    KC_COMMA (KC_COMM) => Code::Comma,
    /// Keyboard `.` and `>`.
    KC_DOT => Code::Dot,
    /// Keyboard `/` and `?`.
    KC_SLASH (KC_SLSH) => Code::Slash,
    /// Keyboard `Caps Lock`.
    KC_CAPSLOCK (KC_CAPS) => Code::CapsLock,
    /// Keyboard `F1`.
    KC_F1 => Code::F1,
    /// Keyboard `F2`.
    KC_F2 => Code::F2,
    /// Keyboard `F3`.
    KC_F3 => Code::F3,
    /// Keyboard `F4`.
    KC_F4 => Code::F4,
    /// Keyboard `F5`.
    KC_F5 => Code::F5,
    /// Keyboard `F6`.
    KC_F6 => Code::F6,
    /// Keyboard `F7`.
    KC_F7 => Code::F7,
    /// Keyboard `F8`.
    KC_F8 => Code::F8,
    /// Keyboard `F9`.
    KC_F9 => Code::F9,
    /// Keyboard `F10`.
    KC_F10 => Code::F10,
    /// Keyboard `F11`.
    KC_F11 => Code::F11,
    /// Keyboard `F12`.
    KC_F12 => Code::F12,
    /// Keyboard `Print Screen`.
    KC_PRINT_SCREEN (KC_PSCR) => Code::PrintScreen,
    /// Keyboard `Scroll Lock`.
    KC_SCROLL_LOCK (KC_SCRL) => Code::ScrollLock,
    /// Keyboard `Pause`.
    KC_PAUSE (KC_PAUS) => Code::Pause,
    /// Keyboard `Insert`.
    KC_INSERT (KC_INS) => Code::Insert,
    /// Keyboard `Home`.
    KC_HOME => Code::Home,
    /// Keyboard `Page Up`.
    KC_PAGE_UP (KC_PGUP) => Code::PageUp,
    /// Keyboard `Delete`.
    KC_DELETE (KC_DEL) => Code::Delete,
    /// Keyboard `End`.
    KC_END => Code::End,
    /// Keyboard `Page Down`.
    KC_PAGE_DOWN (KC_PGDN) => Code::PageDown,
    /// Keyboard `Right Arrow`.
    KC_RIGHT (KC_RGHT) => Code::RightArrow,
    /// Keyboard `Left Arrow`.
    KC_LEFT => Code::LeftArrow,
    /// Keyboard `Down Arrow`.
    KC_DOWN => Code::DownArrow,
    /// Keyboard `Up Arrow`.
    KC_UP => Code::UpArrow,
    /// Keypad `Num Lock`.
    KC_NUM_LOCK (KC_NUM) => Code::KeypadNumLock,
    /// Keypad `/`.
    KC_KP_DIVIDE => Code::KeypadDivide,
    /// Keypad `*`.
    KC_KP_MULTIPLY => Code::KeypadMultiply,
    /// Keypad `-`.
    KC_KP_MINUS => Code::KeypadSubtract,
    /// Keypad `+`.
    KC_KP_PLUS => Code::KeypadAdd,
    /// Keypad `Enter`.
    KC_KP_ENTER => Code::KeypadEnter,
    /// Keypad `1` and `End`.
    KC_KP_1 => Code::Keypad1,
    /// Keypad `2` and `Down Arrow`.
    KC_KP_2 => Code::Keypad2,
    /// Keypad `3` and `Page Down`.
    KC_KP_3 => Code::Keypad3,
    /// Keypad `4` and `Left Arrow`.
    KC_KP_4 => Code::Keypad4,
    /// Keypad `5`.
    KC_KP_5 => Code::Keypad5,
    /// Keypad `6` and `Right Arrow`.
    KC_KP_6 => Code::Keypad6,
    /// Keypad `7` and `Home`.
    KC_KP_7 => Code::Keypad7,
    /// Keypad `8` and `Up Arrow`.
    KC_KP_8 => Code::Keypad8,
    /// Keypad `9` and `Page Up`.
    KC_KP_9 => Code::Keypad9,
    /// Keypad `0` and `Insert`.
    KC_KP_0 => Code::Keypad0,
    /// Keypad `.` and `Delete`.
    KC_KP_DOT => Code::KeypadDot,
    /// Keyboard non-US `\` and `|`.
    KC_NONUS_BACKSLASH (KC_NUBS) => Code::NonUSBackslash,
    /// Keyboard `Application`.
    KC_APPLICATION (KC_APP) => Code::Application,
    /// Keyboard `Power`.
    KC_POWER => Code::Power,
    /// Keyboard `=`.
    KC_KP_EQUAL => Code::KeypadEqual,
    /// Keyboard `F13`.
    KC_F13 => Code::F13,
    /// Keyboard `F14`.
    KC_F14 => Code::F14,
    /// Keyboard `F15`.
    KC_F15 => Code::F15,
    /// Keyboard `F16`.
    KC_F16 => Code::F16,
    /// Keyboard `F17`.
    KC_F17 => Code::F17,
    /// Keyboard `F18`.
    KC_F18 => Code::F18,
    /// Keyboard `F19`.
    KC_F19 => Code::F19,
    /// Keyboard `F20`.
    KC_F20 => Code::F20,
    /// Keyboard `F21`.
    KC_F21 => Code::F21,
    /// Keyboard `F22`.
    KC_F22 => Code::F22,
    /// Keyboard `F23`.
    KC_F23 => Code::F23,
    /// Keyboard `F24`.
    KC_F24 => Code::F24,
    /// Keyboard `Execute`.
    KC_EXECUTE (KC_EXEC) => Code::Execute,
    /// Keyboard `Help`.
    KC_HELP => Code::Help,
    /// Keyboard `Menu`.
    KC_MENU => Code::Menu,
    /// Keyboard `Select`.
    KC_SELECT (KC_SLCT) => Code::Select,
    /// Keyboard `Stop`.
    KC_STOP => Code::Stop,
    /// Keyboard `Again`.
    KC_AGAIN (KC_AGIN) => Code::Again,
    /// Keyboard `Undo`.
    KC_UNDO => Code::Undo,
    /// Keyboard `Cut`.
    KC_CUT => Code::Cut,
    /// Keyboard `Copy`.
    KC_COPY => Code::Copy,
    /// Keyboard `Paste`.
    KC_PASTE (KC_PSTE) => Code::Paste,
    /// Keyboard `Find`.
    KC_FIND => Code::Find,
    /// Keyboard `Mute`.
    KC_MUTE => Code::Mute,
    /// Keyboard `Volume Up`.
    KC_VOLUME_UP (KC_VLUP) => Code::VolumeUp,
    /// Keyboard `Volume Down`.
    KC_VOLUME_DOWN (KC_VLDN) => Code::VolumeDown,
    /// Keyboard `Locking Caps Lock`.
    KC_LOCKING_CAPS_LOCK (KC_LCAP) => Code::LockingCapsLock,
    /// Keyboard `Locking Num Lock`.
    KC_LOCKING_NUM (KC_LNUM) => Code::LockingNumLock,
    /// Keyboard `Locking Scroll Lock`.
    KC_LOCKING_SCROLL (KC_LSCR)=> Code::LockingScrollLock,
    /// Keypad `,`.
    KC_KP_COMMA => Code::KeypadComma,
    /// Keypad `=`.
    KC_KP_EQUAL_AS400 => Code::KeypadEqualSign,
    /// Keyboard `International 1`.
    KC_INTERNATIONAL_1 (KC_INT1) => Code::International1,
    /// Keyboard `International 2`.
    KC_INTERNATIONAL_2 (KC_INT2) => Code::International2,
    /// Keyboard `International 3`.
    KC_INTERNATIONAL_3 (KC_INT3) => Code::International3,
    /// Keyboard `International 4`.
    KC_INTERNATIONAL_4 (KC_INT4) => Code::International4,
    /// Keyboard `International 5`.
    KC_INTERNATIONAL_5 (KC_INT5) => Code::International5,
    /// Keyboard `International 6`.
    KC_INTERNATIONAL_6 (KC_INT6) => Code::International6,
    /// Keyboard `International 7`.
    KC_INTERNATIONAL_7 (KC_INT7) => Code::International7,
    /// Keyboard `International 8`.
    KC_INTERNATIONAL_8 (KC_INT8) => Code::International8,
    /// Keyboard `International 9`.
    KC_INTERNATIONAL_9 (KC_INT9) => Code::International9,
    /// Keyboard `Language 1`.
    KC_LANGUAGE_1 (KC_LANG1) => Code::Language1,
    /// Keyboard `Language 2`.
    KC_LANGUAGE_2 (KC_LANG2) => Code::Language2,
    /// Keyboard `Language 3`.
    KC_LANGUAGE_3 (KC_LANG3) => Code::Language3,
    /// Keyboard `Language 4`.
    KC_LANGUAGE_4 (KC_LANG4) => Code::Language4,
    /// Keyboard `Language 5`.
    KC_LANGUAGE_5 (KC_LANG5) => Code::Language5,
    /// Keyboard `Language 6`.
    KC_LANGUAGE_6 (KC_LANG6) => Code::Language6,
    /// Keyboard `Language 7`.
    KC_LANGUAGE_7 (KC_LANG7) => Code::Language7,
    /// Keyboard `Language 8`.
    KC_LANGUAGE_8 (KC_LANG8) => Code::Language8,
    /// Keyboard `Language 9`.
    KC_LANGUAGE_9 (KC_LANG9) => Code::Language9,
    /// Keyboard `Alternate Erase`.
    KC_ALTERNATE_ERASE (KC_ERAS) => Code::AlternateErase,
    /// Keyboard `SysReq/Attention`.
    KC_SYSREQ => Code::SysReqAttention,
    /// Keyboard `Cancel`.
    KC_CANCEL (KC_CNCL) => Code::Cancel,
    /// Keyboard `Clear`.
    KC_CLEAR (KC_CRL) => Code::Clear,
    /// Keyboard `Prior`.
    KC_PRIOR (KC_PRIR) => Code::Prior,
    /// Keyboard `Return`.
    KC_RETURN (KC_RETN, KC_RET) => Code::Return,
    /// Keyboard `Separator`.
    KC_SEPARATOR (KC_SEPR) => Code::Separator,
    /// Keyboard `Out`.
    KC_OUT => Code::Out,
    /// Keyboard `Oper`.
    KC_OPER => Code::Oper,
    /// Keyboard `Clear/Again`.
    KC_CLEAR_AGAIN (KC_CLAG) => Code::ClearAgain,
    /// Keyboard `CrSel/Props`.
    KC_CRSEL (KC_CRSL) => Code::CrSelProps,
    /// Keyboard `ExSel`.
    KC_EXSEL (KC_EXSL) => Code::ExSel,
    /// Keyboard `Left Control`.
    KC_LEFT_CONTROL (KC_LCTL) => Code::LeftControl,
    /// Keyboard `Left Shift`.
    KC_LEFT_SHIFT (KC_LSFT) => Code::LeftShift,
    /// Keyboard `Left ALT`.
    KC_LEFT_ALT (KC_LALT) => Code::LeftAlt,
    /// Keyboard `Left GUI`.
    KC_LEFT_GUI (KC_LGUI, KC_LCMD, KC_LWIN) => Code::LeftGUI,
    /// Keyboard `Right Control`.
    KC_RIGHT_CONTROL (KC_RCTL) => Code::RightControl,
    /// Keyboard `Right Shift`.
    KC_RIGHT_SHIFT (KC_RSFT) => Code::RightShift,
    /// Keyboard `Right ALT`.
    KC_RIGHT_ALT (KC_RALT) => Code::RightAlt,
    /// Keyboard `Right GUI`.
    KC_RIGHT_GUI (KC_RGUI, KC_RCMD, KC_RWIN) => Code::RightGUI,
}
