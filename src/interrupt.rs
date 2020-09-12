use core::intrinsics::transmute;

/// Uses the magic of monomorphization to "save" the type parameter for our interrupt handler
///
/// Because a new version of this function will be compiled for every H, the function pointer
/// for a monomorphized version of this function will "remember" it's function pointer,
/// allowing us to cast the function pointer into a monomorphized interrupt handler.
pub(crate) fn trampoline<H: Callable>(handler: *mut ()) {
    let handler: &mut H = unsafe { transmute(handler) };
    handler.call();
}

pub(crate) trait Callable {
    fn call(&mut self);
}

macro_rules! int_handler {
    ($INT_TYPE:ident => $INT:ident($DATA:ty)) => {
        use paste::paste;
        use xtensa_lx106_rt::{interrupt, ExceptionContext};
        use xtensa_lx106_rt::interrupt::{enable_interrupt, disable_interrupt, InterruptType};
        use core::pin::Pin;
        use core::intrinsics::transmute;
        use core::marker::PhantomPinned;
        use crate::interrupt::*;

        paste! { static mut [<INT_ $INT_TYPE _TRAMPOLINE>]: Option<fn(handler: *mut ())> = None; }
        paste! { static mut [<INT_ $INT_TYPE _HANDLER>]: *mut () = core::ptr::null_mut(); }

        #[interrupt($INT_TYPE)]
        fn $INT_TYPE(_frame: &ExceptionContext) {
            // this is safe because the drop impl for the handler cleans up the pointers
            unsafe {
                if let Some(trampoline) = paste!{[<INT_ $INT_TYPE _TRAMPOLINE>]} {
                    trampoline(paste!{[<INT_ $INT_TYPE _HANDLER>]})
                }
            }
        }

        pub struct $INT<F> {
            closure: F,
            data: $DATA,
            _pin: PhantomPinned,
        }

        impl<F: FnMut(&mut $DATA)> $INT<F> {
            pub(crate) fn new(data: $DATA, closure: F) -> Pin<Self> {
                enable_interrupt(InterruptType::$INT_TYPE);

                let handler = unsafe {
                    Pin::new_unchecked($INT {
                        data,
                        closure,
                        _pin: PhantomPinned,
                    })
                };
                unsafe {
                    paste!{[<INT_ $INT_TYPE _TRAMPOLINE>] = Some(trampoline::<Self>) };
                    paste!{[<INT_ $INT_TYPE _HANDLER>] = transmute(&handler) };
                }
                handler
            }
        }

        impl<F: FnMut(&mut $DATA)> Callable for $INT<F> {
            fn call(&mut self) {
                (self.closure)(&mut self.data)
            }
        }

        impl<F> Drop for $INT<F> {
            fn drop(&mut self) {
                disable_interrupt(InterruptType:: $INT_TYPE);

                unsafe {
                    paste!{[<INT_ $INT_TYPE _TRAMPOLINE>] = None };
                    paste!{[<INT_ $INT_TYPE _HANDLER>] = core::ptr::null_mut() };
                }
            }
        }

        impl<F> core::ops::Deref for $INT<F> {
            type Target = $DATA;

            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }

        impl<F> core::ops::DerefMut for $INT<F> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.data
            }
        }
    }
}