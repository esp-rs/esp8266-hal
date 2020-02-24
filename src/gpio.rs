use core::marker::PhantomData;
use core::convert::Infallible;

use esp8266::{GPIO, IO_MUX};
use embedded_hal::digital::v2::OutputPin;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;

/// Pulled down input (type state)
pub struct PullDown;

/// Pulled up input (type state)
pub struct PullUp;

/// Open drain input or output (type state)
pub struct OpenDrain;

/// Push pull output (type state)
pub struct PushPull;

/// Analog mode (type state)
pub struct Analog;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

macro_rules! gpio {
    ($GPIO:ident: [
        $($pxi:ident: ($pname:ident, $MODE:ty),)+
    ]) => {

        impl GpioExt for $GPIO {
            type Parts = Parts;

            fn split(self) -> Self::Parts {
                Parts {
                    $(
                        $pname: $pxi { _mode: PhantomData },
                    )+
                }
            }
        }

        pub struct Parts {
            $(
                /// Pin
                pub $pname: $pxi<$MODE>,
            )+
        }

        // create all the pins, we can also add functionality
        // applicable to all pin states here
        $(
            /// Pin
            pub struct $pxi<MODE> {
                _mode: PhantomData<MODE>,
            }
        )+

    };
}

// All info on reset state pulled from 4.10 IO_MUX Pad List in the reference manual
gpio! {
   GPIO: [
       Gpio0: (gpio0, Input<PullUp>),
       Gpio1: (gpio1, Input<PullUp>),
       Gpio2: (gpio2, Input<PullDown>),
       Gpio3: (gpio3, Input<PullUp>),
       Gpio4: (gpio4, Input<PullDown>),
       Gpio5: (gpio5, Input<PullUp>),
       Gpio6: (gpio6, Input<PullUp>),
       Gpio7: (gpio7, Input<PullUp>),
       Gpio8: (gpio8, Input<PullUp>),
       Gpio9: (gpio9, Input<PullUp>),
       Gpio10: (gpio10, Input<PullUp>),
       Gpio11: (gpio11, Input<PullUp>),
       Gpio12: (gpio12, Input<PullDown>),
       Gpio13: (gpio13, Input<Floating>),
       Gpio14: (gpio14, Input<Floating>),
       Gpio15: (gpio15, Input<PullUp>),
   ]
}

macro_rules! impl_output {
    ($en:ident, $outs:ident, $outc:ident, [
        // index, gpio pin name, funcX name, iomux pin name, iomux mcu_sel bits
        $($pxi:ident: ($i:expr, $pin:ident, $iomux:ident, $mcu_sel_bits:expr),)+
    ]) => {
        $(
            impl<MODE> OutputPin for $pxi<Output<MODE>> {
                type Error = Infallible;

                fn set_high(&mut self) -> Result<(), Self::Error> {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*GPIO::ptr()).$outs.write(|w| w.bits(1 << $i)) };
                    Ok(())
                }

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*GPIO::ptr()).$outc.write(|w| w.bits(1 << $i)) };
                    Ok(())
                }
            }

            impl<MODE> $pxi<MODE> {
                pub fn into_push_pull_output(self) -> $pxi<Output<PushPull>> {
                    let gpio = unsafe{ &*GPIO::ptr() };
                    let iomux = unsafe{ &*IO_MUX::ptr() };
                    gpio.$en.modify(|_, w| unsafe  { w.bits(0x1 << $i) });

                    iomux.$iomux.modify(|_, w| unsafe { w.bits($mcu_sel_bits) });
                    $pxi { _mode: PhantomData }
                }
            }
        )+
    };
}

impl_output! {
    gpio_enable_w1ts, gpio_enable_w1tc, gpio_out_w1ts, gpio_out_w1tc, gpio_in, [
        Gpio0: (0, pin0, io_mux_gpio0, 0b00),
        Gpio1: (1, pin1, io_mux_u0txd, 0b11),
        Gpio2: (2, pin2, io_mux_gpio2, 0b00),
        Gpio3: (3, pin3, io_mux_u0rxd, 0b11),
        Gpio4: (4, pin4, io_mux_gpio4, 0b00),
        Gpio5: (5, pin5, io_mux_gpio5, 0b00),
        Gpio6: (6, pin6, io_mux_sd_clk, 0b11),
        Gpio7: (7, pin7, io_mux_sd_data0, 0b11),
        Gpio8: (8, pin8, io_mux_sd_data1, 0b11),
        Gpio9: (9, pin9, io_mux_sd_data2, 0b11),
        Gpio10: (10, pin10, io_mux_sd_data3, 0b11),
        Gpio11: (11, pin11, io_mux_sd_cmd, 0b11),
        Gpio12: (12, pin12, io_mux_mtdi, 0b11),
        Gpio13: (13, pin13, io_mux_mtck, 0b11),
        Gpio14: (14, pin14, io_mux_mtms, 0b11),
        Gpio15: (15, pin15, io_mux_mtdo, 0b11),
    ]
}
