struct Report {
    inner: Inner,
}

impl Report {
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                &self.inner as *const Inner as *const u8,
                core::mem::size_of::<Inner>(),
            )
        }
    }
}

#[repr(C)]
struct Inner {
    mods: u8,
    _reserved: u8,
    foo: [u8; 6],
}
