pub mod usb;

use usb::KeyCode;

pub trait Interface {
    type Handler: Handler;

    fn start(self) -> (Self::Handler, impl Future);
}

pub trait Handler {
    fn register(&mut self, code: KeyCode);
    fn temp_register(&mut self, code: KeyCode);
    fn unregister(&mut self, code: KeyCode);
    fn flush(&mut self);
}
