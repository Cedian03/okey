use crate::action::{Key, Modifier};

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

    pub fn register_code(&mut self, code: Key) -> Result<(), ReportError> {
        if self.len >= self.inner.codes.len() {
            return Err(ReportError)
        }

        self.inner.codes[self.len] = code;
        self.len += 1;

        Ok(())
    }

    pub fn unregister_code(&mut self, code: Key) -> Result<(), ReportError> {
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
        self.inner.codes[self.len] = Key::NoEvent;

        Ok(())
    }

    pub fn register_modifier(&mut self, modifier: Modifier) {
        self.inner.modifiers |= modifier.mask();
    }

    pub fn unregister_modifier(&mut self, modifier: Modifier) {
        self.inner.modifiers &= !modifier.mask();
    }
}

#[derive(Debug)]
#[repr(C)]
struct Inner {
    modifiers: u8,
    _reserved: u8,
    codes: [Key; 6]
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
            codes: [Key::NoEvent; 6],
        }
    }
}