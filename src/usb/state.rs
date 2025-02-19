use embassy_usb::class::hid;

pub struct State<'a> {
    pub(super) config_descriptor_buf: [u8; 256],
    pub(super) bos_descriptor_buf: [u8; 256],
    pub(super) msos_descriptor_buf: [u8; 256],
    pub(super) control_buf: [u8; 64],
    pub(super) hid_state: hid::State<'a>
}

impl<'a> State<'a> {
    pub const fn new() -> Self {
        Self {
            config_descriptor_buf: [0; 256],
            bos_descriptor_buf: [0; 256],
            msos_descriptor_buf: [0; 256],
            control_buf: [0; 64],
            hid_state: hid::State::new(),
        }
    }
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self::new()
    }
}