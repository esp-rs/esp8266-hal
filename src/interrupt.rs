use crate::ram;
use core::intrinsics::transmute;
use xtensa_lx_rt::exception::Context;
use xtensa_lx_rt::interrupt;

#[repr(u8)]
pub enum InterruptType {
    SLC = 1,
    SPI = 2,
    GPIO = 4,
    UART = 5,
    CCOMPARE = 6,
    SOFT = 7,
    WDT = 8,
    TIMER1 = 9,
}

impl InterruptType {
    const fn mask(self) -> u32 {
        1 << self as u8
    }
}

extern "Rust" {
    // interrupt handlers set by #[interrupt] macro
    fn __slc_interrupt(context: &Context);
    fn __spi_interrupt(context: &Context);
    fn __gpio_interrupt(context: &Context);
    fn __uart_interrupt(context: &Context);
    fn __ccompare_interrupt(context: &Context);
    fn __soft_interrupt(context: &Context);
    fn __wdt_interrupt(context: &Context);
    fn __timer1_interrupt(context: &Context);

    // interrupt handlers for the hal to use
    fn __slc_hal_interrupt(context: &Context);
    fn __spi_hal_interrupt(context: &Context);
    fn __gpio_hal_interrupt(context: &Context);
    fn __uart_hal_interrupt(context: &Context);
    fn __ccompare_hal_interrupt(context: &Context);
    fn __soft_hal_interrupt(context: &Context);
    fn __wdt_hal_interrupt(context: &Context);
    fn __timer1_hal_interrupt(context: &Context);
}

#[interrupt]
#[ram]
fn interrupt_trampoline(level: u32, frame: Context) {
    let _ = level;
    let mask = xtensa_lx::interrupt::get();
    unsafe {
        xtensa_lx::interrupt::clear(mask);
    }
    unsafe {
        if InterruptType::SLC.mask() & mask > 0 {
            __slc_interrupt(&frame);
            __slc_hal_interrupt(&frame);
        };
        if InterruptType::SPI.mask() & mask > 0 {
            __spi_interrupt(&frame);
            __spi_hal_interrupt(&frame);
        };
        if InterruptType::GPIO.mask() & mask > 0 {
            __gpio_interrupt(&frame);
            __gpio_hal_interrupt(&frame);
        };
        if InterruptType::UART.mask() & mask > 0 {
            __uart_interrupt(&frame);
            __uart_hal_interrupt(&frame);
        };
        if InterruptType::CCOMPARE.mask() & mask > 0 {
            __ccompare_interrupt(&frame);
            __ccompare_hal_interrupt(&frame);
        };
        if InterruptType::SOFT.mask() & mask > 0 {
            __soft_interrupt(&frame);
            __soft_hal_interrupt(&frame);
        };
        if InterruptType::WDT.mask() & mask > 0 {
            __wdt_interrupt(&frame);
            __wdt_hal_interrupt(&frame);
        };
        if InterruptType::TIMER1.mask() & mask > 0 {
            __timer1_interrupt(&frame);
            __timer1_hal_interrupt(&frame);
        };
    }
}

pub fn enable_interrupt(ty: InterruptType) -> u32 {
    let type_mask = ty.mask();
    unsafe { xtensa_lx::interrupt::enable_mask(type_mask) }
}

pub fn disable_interrupt(ty: InterruptType) -> u32 {
    let type_mask = !(1u32 << ty as u8);
    unsafe { xtensa_lx::interrupt::enable_mask(type_mask) }
}

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

/// Setup an interrupt handler accepting a closure
macro_rules! int_handler {
    ($INT_TYPE:ident => $INT:ident($DATA:ty)) => {
        use crate::interrupt::*;
        use core::intrinsics::transmute;
        use core::marker::PhantomPinned;
        use core::pin::Pin;
        use paste::paste;
        use xtensa_lx_rt::exception::Context;

        paste! { static mut [<INT_ $INT_TYPE _TRAMPOLINE>]: Option<fn(handler: *mut ())> = None; }
        paste! { static mut [<INT_ $INT_TYPE _HANDLER>]: *mut () = core::ptr::null_mut(); }

        paste! {
            #[no_mangle]
            fn [<__ $INT_TYPE:lower _hal_interrupt>](_frame: &Context) {
                // this is safe because the drop impl for the handler cleans up the pointers
                unsafe {
                    if let Some(trampoline) = paste!{[<INT_ $INT_TYPE _TRAMPOLINE>]} {
                        trampoline(paste!{[<INT_ $INT_TYPE _HANDLER>]})
                    }
                }
            }
        }

        pub struct $INT<F> {
            closure: F,
            data: $DATA,
            _pin: PhantomPinned,
        }

        impl<F: FnMut(&mut $DATA)> $INT<F> {
            /// Create interrupt handler that keeps the closure in scope and sets the static pointers
            ///
            /// returns the handler pinned to ensure that the pointers stay valid
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
                    paste! {[<INT_ $INT_TYPE _TRAMPOLINE>] = Some(trampoline::<Self>) };
                    paste! {[<INT_ $INT_TYPE _HANDLER>] = transmute(&handler) };
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
                disable_interrupt(InterruptType::$INT_TYPE);

                unsafe {
                    paste! {[<INT_ $INT_TYPE _TRAMPOLINE>] = None };
                    paste! {[<INT_ $INT_TYPE _HANDLER>] = core::ptr::null_mut() };
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
    };
}
