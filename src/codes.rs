//! QMK/TMK style keycodes for ease of configuration.

use crate::action::{Action, Key, Modifier};

/// Transparent.
pub const _______: Option<Action> = None;
/// No action. 
pub const XXXXXXX: Option<Action> = Some(Action::NoAction);

macro_rules! define_keys {
    ($(#[doc = $doc:literal] $name:ident $(($($alias:ident),*))? => $key:ident),* $(,)?) => {
        $(
            #[doc = $doc]            
            pub const $name: Option<Action> = Some(Action::Key(Key::$key));

            $($(
                #[doc = $doc]
                #[doc = concat!(r"Alias for [`", stringify!($name), "`]")]          
                pub const $alias: Option<Action> = $name;
            )*)?
        )*
    };
}

define_keys! {
    #[doc = r"Keyboard `a` and `A`."]
    KC_A => KeyboardA,
    #[doc = r"Keyboard `b` and `B`."]
    KC_B => KeyboardB,
    #[doc = r"Keyboard `c` and `C`."]
    KC_C => KeyboardC,
    #[doc = r"Keyboard `d` and `D`."]
    KC_D => KeyboardD,
    #[doc = r"Keyboard `e` and `E`."]
    KC_E => KeyboardE,
    #[doc = r"Keyboard `f` and `F`."]
    KC_F => KeyboardF,
    #[doc = r"Keyboard `g` and `G`."]
    KC_G => KeyboardG,
    #[doc = r"Keyboard `h` and `H`."]
    KC_H => KeyboardH,
    #[doc = r"Keyboard `i` and `I`."]
    KC_I => KeyboardI,
    #[doc = r"Keyboard `j` and `J`."]
    KC_J => KeyboardJ,
    #[doc = r"Keyboard `k` and `K`."]
    KC_K => KeyboardK,
    #[doc = r"Keyboard `l` and `L`."]
    KC_L => KeyboardL,
    #[doc = r"Keyboard `m` and `M`."]
    KC_M => KeyboardM,
    #[doc = r"Keyboard `n` and `N`."]
    KC_N => KeyboardN,
    #[doc = r"Keyboard `o` and `O`."]
    KC_O => KeyboardO,
    #[doc = r"Keyboard `p` and `P`."]
    KC_P => KeyboardP,
    #[doc = r"Keyboard `q` and `Q`."]
    KC_Q => KeyboardQ,
    #[doc = r"Keyboard `r` and `R`."]
    KC_R => KeyboardR,
    #[doc = r"Keyboard `s` and `S`."]
    KC_S => KeyboardS,
    #[doc = r"Keyboard `t` and `T`."]
    KC_T => KeyboardT,
    #[doc = r"Keyboard `u` and `U`."]
    KC_U => KeyboardU,
    #[doc = r"Keyboard `v` and `V`."]
    KC_V => KeyboardV,
    #[doc = r"Keyboard `w` and `W`."]
    KC_W => KeyboardW,
    #[doc = r"Keyboard `x` and `X`."]
    KC_X => KeyboardX,
    #[doc = r"Keyboard `y` and `Y`."]
    KC_Y => KeyboardY,
    #[doc = r"Keyboard `z` and `Z`."]
    KC_Z => KeyboardZ,
    #[doc = r"Keyboard `1` and `!`."]
    KC_1 => Keyboard1,
    #[doc = r"Keyboard `2` and `@`."]
    KC_2 => Keyboard2,
    #[doc = r"Keyboard `3` and `#`."]
    KC_3 => Keyboard3,
    #[doc = r"Keyboard `4` and `$`."]
    KC_4 => Keyboard4,
    #[doc = r"Keyboard `5` and `%`."]
    KC_5 => Keyboard5,
    #[doc = r"Keyboard `6` and `^`."]
    KC_6 => Keyboard6,
    #[doc = r"Keyboard `7` and `&`."]
    KC_7 => Keyboard7,
    #[doc = r"Keyboard `8` and `*`."]
    KC_8 => Keyboard8,
    #[doc = r"Keyboard `9` and `(`."]
    KC_9 => Keyboard9,
    #[doc = r"Keyboard `0` and `)`."]
    KC_0 => Keyboard0,
    #[doc = r"Keyboard `Enter`."]
    KC_ENTER (KC_ENTR, KC_ENT) => Enter,
    #[doc = r"Keyboard `Escape`."]
    KC_ESCAPE (KC_ESC) => Escape,
    #[doc = r"Keyboard `Backspace`."]
    KC_BACKSPACE (KC_BSPC) => Backspace,
    #[doc = r"Keyboard `Tab`."]
    KC_TAB => Tab,
    #[doc = r"Keyboard `Space`."]
    KC_SPACE (KC_SPC) => Space,
    #[doc = r"Keyboard `-` and `_`."]
    KC_MINUS (KC_MINS) => Minus,
    #[doc = r"Keyboard `=` and `+`."]
    KC_EQUAL (KC_EQL) => Equal,
    #[doc = r"Keyboard `[` and `{`."]
    KC_LEFT_BRACKET (KC_LBRC) => LeftBracket,
    #[doc = r"Keyboard `]` and `}`."]
    KC_RIGHT_BRACKET (KC_RBRC) => RightBracket,
    #[doc = r"Keyboard `\` and `|`."]
    KC_BACKSLASH (KC_BSLS) => Backslash,
    #[doc = r"Keyboard non-US `#` and `~`."]
    KC_NONUS_HASH (KC_NUHS) => NonUSHash,
    #[doc = r"Keyboard `;` and `:`."]
    KC_SEMICOLON (KC_SCLN) => Semicolon,
    #[doc = r#"Keyboard `'` and `"`."#]
    KC_QUOTE (KC_QOUT) => Apostrophe,
    #[doc = r"Keyboard `` ` `` and `~`."]
    KC_GRAVE (KC_GRV) => Grave,
    #[doc = r"Keyboard `,` and `<`."]
    KC_COMMA (KC_COMM) => Comma,
    #[doc = r"Keyboard `.` and `>`."]
    KC_DOT => Dot,
    #[doc = r"Keyboard `/` and `?`."]
    KC_SLASH (KC_SLSH) => Slash,
    #[doc = r"Keyboard `Caps Lock`."]
    KC_CAPSLOCK (KC_CAPS) => CapsLock,
    #[doc = r"Keyboard `F1`."]
    KC_F1 => F1,
    #[doc = r"Keyboard `F2`."]
    KC_F2 => F2,
    #[doc = r"Keyboard `F3`."]
    KC_F3 => F3,
    #[doc = r"Keyboard `F4`."]
    KC_F4 => F4,
    #[doc = r"Keyboard `F5`."]
    KC_F5 => F5,
    #[doc = r"Keyboard `F6`."]
    KC_F6 => F6,
    #[doc = r"Keyboard `F7`."]
    KC_F7 => F7,
    #[doc = r"Keyboard `F8`."]
    KC_F8 => F8,
    #[doc = r"Keyboard `F9`."]
    KC_F9 => F9,
    #[doc = r"Keyboard `F10`."]
    KC_F10 => F10,
    #[doc = r"Keyboard `F11`."]
    KC_F11 => F11,
    #[doc = r"Keyboard `F12`."]
    KC_F12 => F12,
    #[doc = r"Keyboard `Print Screen`."]
    KC_PRINT_SCREEN (KC_PSCR) => PrintScreen,
    #[doc = r"Keyboard `Scroll Lock`."]
    KC_SCROLL_LOCK (KC_SCRL) => ScrollLock,
    #[doc = r"Keyboard `Pause`."]
    KC_PAUSE (KC_PAUS) => Pause,
    #[doc = r"Keyboard `Insert`."]
    KC_INSERT (KC_INS) => Insert,
    #[doc = r"Keyboard `Home`."]
    KC_HOME => Home,
    #[doc = r"Keyboard `Page Up`."]
    KC_PAGE_UP (KC_PGUP) => PageUp,
    #[doc = r"Keyboard `Delete`."]
    KC_DELETE (KC_DEL) => Delete,
    #[doc = r"Keyboard `End`."]
    KC_END => End,
    #[doc = r"Keyboard `Page Down`."]
    KC_PAGE_DOWN (KC_PGDN) => PageDown,
    #[doc = r"Keyboard `Right Arrow`."]
    KC_RIGHT (KC_RGHT) => RightArrow,
    #[doc = r"Keyboard `Left Arrow`."]
    KC_LEFT => LeftArrow,
    #[doc = r"Keyboard `Down Arrow`."]
    KC_DOWN => DownArrow,
    #[doc = r"Keyboard `Up Arrow`."]
    KC_UP => UpArrow,
    #[doc = r"Keypad `Num Lock`."]
    KC_NUM_LOCK (KC_NUM) => KeypadNumLock,
    #[doc = r"Keypad `/`."]
    KC_KP_DIVIDE => KeypadDivide,
    #[doc = r"Keypad `*`."]
    KC_KP_MULTIPLY => KeypadMultiply,
    #[doc = r"Keypad `-`."]
    KC_KP_MINUS => KeypadSubtract,
    #[doc = r"Keypad `+`."]
    KC_KP_PLUS => KeypadAdd,
    #[doc = r"Keypad `Enter`."]
    KC_KP_ENTER => KeypadEnter,
    #[doc = r"Keypad `1` and `End`."]
    KC_KP_1 => Keypad1,
    #[doc = r"Keypad `2` and `Down Arrow`."]
    KC_KP_2 => Keypad2,
    #[doc = r"Keypad `3` and `Page Down`."]
    KC_KP_3 => Keypad3,
    #[doc = r"Keypad `4` and `Left Arrow`."]
    KC_KP_4 => Keypad4,
    #[doc = r"Keypad `5`."]
    KC_KP_5 => Keypad5,
    #[doc = r"Keypad `6` and `Right Arrow`."]
    KC_KP_6 => Keypad6,
    #[doc = r"Keypad `7` and `Home`."]
    KC_KP_7 => Keypad7,
    #[doc = r"Keypad `8` and `Up Arrow`."]
    KC_KP_8 => Keypad8,
    #[doc = r"Keypad `9` and `Page Up`."]
    KC_KP_9 => Keypad9,
    #[doc = r"Keypad `0` and `Insert`."]
    KC_KP_0 => Keypad0,
    #[doc = r"Keypad `.` and `Delete`."]
    KC_KP_DOT => KeypadDot,
    #[doc = r"Keyboard non-US `\` and `|`."]
    KC_NONUS_BACKSLASH (KC_NUBS) => NonUSBackslash,
    #[doc = r"Keyboard `Application`."]
    KC_APPLICATION (KC_APP) => Application,
    #[doc = r"Keyboard `Power`."]
    KC_POWER => Power,
    #[doc = r"Keyboard `=`."]
    KC_KP_EQUAL => KeypadEqual,
    #[doc = r"Keyboard `F13`."]
    KC_F13 => F13,
    #[doc = r"Keyboard `F14`."]
    KC_F14 => F14,
    #[doc = r"Keyboard `F15`."]
    KC_F15 => F15,
    #[doc = r"Keyboard `F16`."]
    KC_F16 => F16,
    #[doc = r"Keyboard `F17`."]
    KC_F17 => F17,
    #[doc = r"Keyboard `F18`."]
    KC_F18 => F18,
    #[doc = r"Keyboard `F19`."]
    KC_F19 => F19,
    #[doc = r"Keyboard `F20`."]
    KC_F20 => F20,
    #[doc = r"Keyboard `F21`."]
    KC_F21 => F21,
    #[doc = r"Keyboard `F22`."]
    KC_F22 => F22,
    #[doc = r"Keyboard `F23`."]
    KC_F23 => F23,
    #[doc = r"Keyboard `F24`."]
    KC_F24 => F24,
    #[doc = r"Keyboard `Execute`."]
    KC_EXECUTE (KC_EXEC) => Execute,
    #[doc = r"Keyboard `Help`."]
    KC_HELP => Help,
    #[doc = r"Keyboard `Menu`."]
    KC_MENU => Menu,
    #[doc = r"Keyboard `Select`."]
    KC_SELECT (KC_SLCT) => Select,
    #[doc = r"Keyboard `Stop`."]
    KC_STOP => Stop,
    #[doc = r"Keyboard `Again`."]
    KC_AGAIN (KC_AGIN) => Again,
    #[doc = r"Keyboard `Undo`."]
    KC_UNDO => Undo,
    #[doc = r"Keyboard `Cut`."]
    KC_CUT => Cut,
    #[doc = r"Keyboard `Copy`."]
    KC_COPY => Copy,
    #[doc = r"Keyboard `Paste`."]
    KC_PASTE (KC_PSTE) => Paste,
    #[doc = r"Keyboard `Find`."]
    KC_FIND => Find,
    #[doc = r"Keyboard `Mute`."]
    KC_MUTE => Mute,
    #[doc = r"Keyboard `Volume Up`."]
    KC_VOLUME_UP (KC_VLUP) => VolumeUp,
    #[doc = r"Keyboard `Volume Down`."]
    KC_VOLUME_DOWN (KC_VLDN) => VolumeDown,
    #[doc = r"Keyboard `Locking Caps Lock`."]
    KC_LOCKING_CAPS_LOCK (KC_LCAP) => LockingCapsLock,
    #[doc = r"Keyboard `Locking Num Lock`."]
    KC_LOCKING_NUM (KC_LNUM) => LockingNumLock,
    #[doc = r"Keyboard `Locking Scroll Lock`."]
    KC_LOCKING_SCROLL (KC_LSCR)=> LockingScrollLock,
    #[doc = r"Keypad `,`."]
    KC_KP_COMMA => KeypadComma,
    #[doc = r"Keypad `=`."]
    KC_KP_EQUAL_AS400 => KeypadEqualSign,
    #[doc = r"Keyboard `International 1`."]
    KC_INTERNATIONAL_1 (KC_INT1) => International1,
    #[doc = r"Keyboard `International 2`."]
    KC_INTERNATIONAL_2 (KC_INT2) => International2,
    #[doc = r"Keyboard `International 3`."]
    KC_INTERNATIONAL_3 (KC_INT3) => International3,
    #[doc = r"Keyboard `International 4`."]
    KC_INTERNATIONAL_4 (KC_INT4) => International4,
    #[doc = r"Keyboard `International 5`."]
    KC_INTERNATIONAL_5 (KC_INT5) => International5,
    #[doc = r"Keyboard `International 6`."]
    KC_INTERNATIONAL_6 (KC_INT6) => International6,
    #[doc = r"Keyboard `International 7`."]
    KC_INTERNATIONAL_7 (KC_INT7) => International7,
    #[doc = r"Keyboard `International 8`."]
    KC_INTERNATIONAL_8 (KC_INT8) => International8,
    #[doc = r"Keyboard `International 9`."]
    KC_INTERNATIONAL_9 (KC_INT9) => International9,
    #[doc = r"Keyboard `Language 1`."]
    KC_LANGUAGE_1 (KC_LANG1) => Language1,
    #[doc = r"Keyboard `Language 2`."]
    KC_LANGUAGE_2 (KC_LANG2) => Language2,
    #[doc = r"Keyboard `Language 3`."]
    KC_LANGUAGE_3 (KC_LANG3) => Language3,
    #[doc = r"Keyboard `Language 4`."]
    KC_LANGUAGE_4 (KC_LANG4) => Language4,
    #[doc = r"Keyboard `Language 5`."]
    KC_LANGUAGE_5 (KC_LANG5) => Language5,
    #[doc = r"Keyboard `Language 6`."]
    KC_LANGUAGE_6 (KC_LANG6) => Language6,
    #[doc = r"Keyboard `Language 7`."]
    KC_LANGUAGE_7 (KC_LANG7) => Language7,
    #[doc = r"Keyboard `Language 8`."]
    KC_LANGUAGE_8 (KC_LANG8) => Language8,
    #[doc = r"Keyboard `Language 9`."]
    KC_LANGUAGE_9 (KC_LANG9) => Language9,
    #[doc = r"Keyboard `Alternate Erase`. "]
    KC_ALTERNATE_ERASE (KC_ERAS) => AlternateErase,
    #[doc = r"Keyboard `SysReq/Attention`. "]
    KC_SYSREQ => SysReqAttention,
    #[doc = r"Keyboard `Cancel`. "]
    KC_CANCEL (KC_CNCL) => Cancel,
    #[doc = r"Keyboard `Clear`. "]
    KC_CLEAR (KC_CRL) => Clear,
    #[doc = r"Keyboard `Prior`. "]
    KC_PRIOR (KC_PRIR) => Prior,
    #[doc = r"Keyboard `Return`. "]
    KC_RETURN (KC_RETN, KC_RET) => Return,
    #[doc = r"Keyboard `Separator`. "]
    KC_SEPARATOR (KC_SEPR) => Separator,
    #[doc = r"Keyboard `Out`. "]
    KC_OUT => Out,
    #[doc = r"Keyboard `Oper`. "]
    KC_OPER => Oper,
    #[doc = r"Keyboard `Clear/Again`. "]
    KC_CLEAR_AGAIN (KC_CLAG) => ClearAgain,
    #[doc = r"Keyboard `CrSel/Props`. "]
    KC_CRSEL (KC_CRSL) => CrSelProps,
    #[doc = r"Keyboard `ExSel`. "]
    KC_EXSEL (KC_EXSL) => ExSel,
}

macro_rules! define_modifiers {
    ($(#[doc = $doc:literal] $name:ident $(($($alias:ident),*))? => $modifier:ident),* $(,)?) => {
        $(
            #[doc = $doc]            
            pub const $name: Option<Action> = Some(Action::Modifier(Modifier::$modifier));

            $($(
                #[doc = $doc]
                #[doc = concat!(r"Alias for [`", stringify!($name), "`]")]          
                pub const $alias: Option<Action> = $name;
            )*)?
        )*
    };
}

define_modifiers! {
    #[doc = r"Keyboard `Left Control`."]
    KC_LEFT_CONTROL (KC_LCTL) => LeftControl,
    #[doc = r"Keyboard `Left Shift`."]
    KC_LEFT_SHIFT (KC_LSFT) => LeftShift,
    #[doc = r"Keyboard `Left ALT`."]
    KC_LEFT_ALT (KC_LALT) => LeftAlt,
    #[doc = r"Keyboard `Left GUI`."]
    KC_LEFT_GUI (KC_LGUI, KC_LCMD, KC_LWIN) => LeftGUI,
    #[doc = r"Keyboard `Right Control`."]
    KC_RIGHT_CONTROL (KC_RCTL) => RightControl,
    #[doc = r"Keyboard `Right Shift`."]
    KC_RIGHT_SHIFT (KC_RSFT) => RightShift,
    #[doc = r"Keyboard `Right ALT`."]
    KC_RIGHT_ALT (KC_RALT) => RightAlt,
    #[doc = r"Keyboard `Right GUI`."]
    KC_RIGHT_GUI (KC_RGUI, KC_RCMD, KC_RWIN) => RightGUI,
}

/// Activate `layer` while key is being held.
#[allow(non_snake_case)]
pub const fn MT(layer: u8) -> Option<Action> {
    Some(Action::MomentaryLayer(layer))
}

/// Toggle active status of `layer`.
#[allow(non_snake_case)]
pub const fn TG(layer: u8) -> Option<Action> {
    Some(Action::ToggleLayer(layer))
}
