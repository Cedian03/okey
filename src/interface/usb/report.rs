use super::Code;

/// No event indicated.
pub const NO_EVENT: u8 = 0x00;
// /// Roll-over error.
// pub const ROLL_OVER_ERROR: u8 = 0x01;
// /// Post fail error.
// pub const POST_FAIL: u8 = 0x02;
// /// Undefined error.
// pub const UNDEFINED_ERROR: u8 = 0x03;

pub const REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x01,  // Usage Page (Generic Desktop)
    0x09, 0x06,  // Usage (Keyboard)
    0xA1, 0x01,  // Collection (Application)
    0x05, 0x07,  //   Usage Page (Key Codes)
    0x19, 0xE0,  //   Usage Minimum (224)
    0x29, 0xE7,  //   Usage Maximum (231)
    0x15, 0x00,  //   Logical Minimum (0)
    0x25, 0x01,  //   Logical Minimum (1)
    0x75, 0x01,  //   Report Size (1)
    0x95, 0x08,  //   Report Count (8)
    0x81, 0x02,  //   Input (Data, Variable, Absolute)
    0x95, 0x01,  //   Report Count (1)
    0x75, 0x08,  //   Report Size (8)
    0x81, 0x01,  //   Input (Constant)
    0x95, 0x05,  //   Report Count (5)
    0x75, 0x01,  //   Report Size (1)
    0x05, 0x08,  //   Usage Page (Page# for LEDs)
    0x19, 0x01,  //   Usage Minimum (1)
    0x29, 0x05,  //   Usage Maximum (5)
    0x91, 0x02,  //   Output (Data, Variable, Absolute)
    0x95, 0x01,  //   Report Count (1)
    0x75, 0x03,  //   Report Size (3)
    0x91, 0x01,  //   Output (Constant)
    0x95, 0x06,  //   Report Count (6)
    0x75, 0x08,  //   Report Size (8)
    0x15, 0x00,  //   Logical Minimum (0)
    0x25, 0x65,  //   Logical Maximum(101)
    0x05, 0x07,  //   Usage Page (Key Codes)
    0x19, 0x00,  //   Usage Minimum (0)
    0x29, 0x65,  //   Usage Maximum (101)
    0x81, 0x00,  //   Input (Data, Array)
    0xC0         // End Collection
]; 

#[derive(Clone, Copy, Debug)]
pub struct ReportError;

#[derive(Clone, Copy, Debug, Default)]
pub struct Report {
    inner: Inner,
    len: usize,
}

impl Report {
    pub const fn new() -> Self {
        Self {
            inner: Inner::new(),
            len: 0,
        }
    }

    pub const fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    pub fn add(&mut self, code: Code) -> Result<(), ReportError> {
        if let Some(mask) = code.modifier_mask() {
            self.inner.modifiers |= mask
        } else {
            if self.len >= self.inner.codes.len() {
                return Err(ReportError)
            }
    
            self.inner.codes[self.len] = code as u8;
            self.len += 1;
        }

        Ok(())
    }

    pub fn remove(&mut self, code: Code) -> Result<(), ReportError> {
        if let Some(mask) = code.modifier_mask() {
            self.inner.modifiers &= !mask
        } else {
            let code = code as u8;

            let mut i = 0;
    
            while i < self.len && self.inner.codes[i] != code {
                i += 1;
            }
    
            if self.inner.codes[i] != code {
                return Err(ReportError);
            }
    
            while i < self.len - 1  {
                self.inner.codes.swap(i, i + 1);
                i += 1;
            }
    
            self.len -= 1;
            self.inner.codes[self.len] = NO_EVENT;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Inner {
    modifiers: u8,
    _reserved: u8,
    codes: [u8; 6]
}

impl Inner {
    const fn new() -> Self {
        Self {
            modifiers: 0,
            _reserved: 0,
            codes: [NO_EVENT; 6],
        }
    }

    const fn as_slice(&self) -> &[u8] {
        // TODO: Safety comment
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
        Self::new()
    }
}