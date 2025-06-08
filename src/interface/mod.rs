pub mod usb;

use usb::Code;

pub trait Interface {
    type Handler: Handler;

    fn start(self) -> (Self::Handler, impl Future);
}

pub trait Handler {
    fn register(&mut self, code: Code);
    fn temp_register(&mut self, code: Code);
    fn unregister(&mut self, code: Code);
    fn flush(&mut self);
}
