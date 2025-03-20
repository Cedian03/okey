pub mod usb;

use usb::Code;

pub trait Interface
{
    type Handler: Handler;

    fn start(self) -> (Self::Handler, impl Future);
}

pub trait Handler {
    fn register_code(&mut self, code: Code);
    fn unregister_code(&mut self, code: Code);
    fn temp_register_code(&mut self, code: Code);

    fn ready(&mut self) -> impl Future<Output = ()>;
    fn flush(&mut self) -> impl Future<Output = ()> { async {} }
}