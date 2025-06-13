mod kb;
mod usb;

// pub enum UsbAction {
//     A, B, C
// }

// pub struct UsbHandler;

// impl Handler for UsbHandler {
//     type Action = UsbAction;

//     fn handle(&mut self, action: Self::Action) {
//         todo!()
//     }
// }

// pub enum FooAction<A> {
//     Inner(A),
//     Foo1,
//     Foo2,
// }

// pub struct FooHandler<H> {
//     inner: H,
// }

// impl<H: Handler> Handler for FooHandler<H> {
//     type Action = FooAction<H::Action>;

//     fn handle(&mut self, action: Self::Action) {
//         match action {
//             FooAction::Inner(e) => self.inner.handle(e),
//             FooAction::Foo1 => { /* Foo1 */ }
//             FooAction::Foo2 => { /* Foo2 */ }
//         }
//     }
// }

// impl<H: Handler> Wrap<H> for FooHandler<H> {
//     fn wrap(other: H) -> Self {
//         Self { inner: other }
//     }
// }

// pub enum BarAction<A> {
//     Inner(A),
//     Bar1,
//     Bar2,
// }

// pub struct BarHandler<H> {
//     inner: H,
// }

// impl<H: Handler> Handler for BarHandler<H> {
//     type Action = BarAction<H::Action>;

//     fn handle(&mut self, action: Self::Action) {
//         match action {
//             BarAction::Inner(e) => self.inner.handle(e),
//             BarAction::Bar1 => { /* Bar1 */ }
//             BarAction::Bar2 => { /* Bar2 */ }
//         }
//     }
// }

// impl<H: Handler> Wrap<H> for BarHandler<H> {
//     fn wrap(other: H) -> Self {
//         Self { inner: other }
//     }
// }
