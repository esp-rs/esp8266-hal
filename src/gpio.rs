use core::convert::Infallible;
use core::marker::PhantomData;

use embedded_hal::digital::v2::{InputPin, OutputPin};
use esp8266::{GPIO, IO_MUX};

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

/// Uninitialized mode (type state)
pub struct UnInitialized {}

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

gpio! {
   GPIO: [
       Gpio0: (gpio0, Input<UnInitialized>),
       Gpio1: (gpio1, Input<UnInitialized>),
       Gpio2: (gpio2, Input<UnInitialized>),
       Gpio3: (gpio3, Input<UnInitialized>),
       Gpio4: (gpio4, Input<UnInitialized>),
       Gpio5: (gpio5, Input<UnInitialized>),
       Gpio6: (gpio6, Input<UnInitialized>),
       Gpio7: (gpio7, Input<UnInitialized>),
       Gpio8: (gpio8, Input<UnInitialized>),
       Gpio9: (gpio9, Input<UnInitialized>),
       Gpio10: (gpio10, Input<UnInitialized>),
       Gpio11: (gpio11, Input<UnInitialized>),
       Gpio12: (gpio12, Input<UnInitialized>),
       Gpio13: (gpio13, Input<UnInitialized>),
       Gpio14: (gpio14, Input<UnInitialized>),
       Gpio15: (gpio15, Input<UnInitialized>),
   ]
}

macro_rules! impl_output {
    ($en:ident, $dis:ident, $outs:ident, $outc:ident, $in:ident, [
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

            impl<MODE> InputPin for $pxi<Input<MODE>> {
                type Error = Infallible;

                fn is_high(&self) -> Result<bool, Self::Error> {
                    let input = unsafe { (*GPIO::ptr()).$in.read().gpio_in_data().bits() };
                    Ok((input >> $i) & 1 == 1)
                }

                fn is_low(&self) -> Result<bool, Self::Error> {
                    let input = unsafe { (*GPIO::ptr()).$in.read().gpio_in_data().bits() };
                    Ok((input >> $i) & 1 == 0)
                }
            }

            impl<MODE> $pxi<MODE> {
                pub fn into_push_pull_output(self) -> $pxi<Output<PushPull>> {
                    let gpio = unsafe{ &*GPIO::ptr() };
                    let iomux = unsafe{ &*IO_MUX::ptr() };
                    iomux.$iomux.modify(|_, w| unsafe {
                        w.function_select_low_bits().bits($mcu_sel_bits & 0b11)
                        .function_select_high_bit().bit($mcu_sel_bits >> 2 == 1)
                    });

                    gpio.$en.modify(|_, w| unsafe { w.bits(0x1 << $i) });

                    $pxi { _mode: PhantomData }
                }

                pub fn into_floating_input(self) -> $pxi<Input<Floating>> {
                    let gpio = unsafe{ &*GPIO::ptr() };
                    let iomux = unsafe{ &*IO_MUX::ptr() };
                    iomux.$iomux.modify(|_, w| unsafe {
                        w.function_select_low_bits().bits($mcu_sel_bits & 0b11)
                        .function_select_high_bit().bit($mcu_sel_bits >> 2 == 1)
                        .pullup().clear_bit()
                    });

                    gpio.$dis.modify(|_, w| unsafe { w.bits(0x1 << $i) });

                    $pxi { _mode: PhantomData }
                }

                pub fn into_pull_up_input(self) -> $pxi<Input<PullUp>> {
                    let gpio = unsafe{ &*GPIO::ptr() };
                    let iomux = unsafe{ &*IO_MUX::ptr() };
                    iomux.$iomux.modify(|_, w| unsafe {
                        w.function_select_low_bits().bits($mcu_sel_bits & 0b11)
                        .function_select_high_bit().bit($mcu_sel_bits >> 2 == 1)
                        .pullup().set_bit()
                    });

                    gpio.$dis.modify(|_, w| unsafe { w.bits(0x1 << $i) });

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
