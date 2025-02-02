use crate::keycode::KeyCode;

#[repr(C)]
pub struct Report {
    modifiers: u8,
    _reserved: u8,
    keycodes: [KeyCode; 6]
}

impl Report {
    pub fn new(modifiers: u8, keycodes: [KeyCode; 6]) -> Self {
        Self {
            modifiers,
            _reserved: 0,
            keycodes,
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { 
            core::slice::from_raw_parts(
                self as *const Self as *const u8, 
                core::mem::size_of::<Self>()
            ) 
        }
    }
}
