use crate::keycode::KeyCode;

pub struct ReportError;

#[derive(Debug, Default)]
pub struct Report {
    inner: Inner,
    len: usize,
}

impl Report {
    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    pub fn register_code(&mut self, code: KeyCode) -> Result<(), ReportError> {
        if self.len >= self.inner.codes.len() {
            return Err(ReportError)
        }

        self.inner.codes[self.len] = code;
        self.len += 1;

        Ok(())
    }

    pub fn unregister_code(&mut self, code: KeyCode) -> Result<(), ReportError> {
        let mut i = 0;

        while i < self.len && self.inner.codes[i] != code {
            i += 1;
        }

        if self.inner.codes[i] != code {
            return Err(ReportError);
        }

        while i < self.len - 1  {
            self.inner.codes.swap(i, i + 1);
        }

        self.len -= 1;
        self.inner.codes[self.len] = KeyCode::NoEvent;

        Ok(())
    }

    pub fn register_modifier(&mut self, mask: u8) {
        self.inner.modifiers |= mask;
    }

    pub fn unregister_modifier(&mut self, mask: u8) {
        self.inner.modifiers &= !mask;
    }
}

#[derive(Debug)]
#[repr(C)]
struct Inner {
    modifiers: u8,
    _reserved: u8,
    codes: [KeyCode; 6]
}

impl Inner {
    pub fn as_slice(&self) -> &[u8] {
        unsafe { 
            core::slice::from_raw_parts(
                self as *const Self as *const u8, 
                core::mem::size_of::<Self>()
            ) 
        }
    }
}

impl Default for Inner {
    fn default() -> Self {
        Self {
            modifiers: 0,
            _reserved: 0,
            codes: [KeyCode::NoEvent; 6],
        }
    }
}