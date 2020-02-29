use core::convert::Infallible;
use core::marker::PhantomData;

use embedded_hal::digital::v2::{toggleable, InputPin, OutputPin, StatefulOutputPin};
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

/// Pin is used by UART
pub struct UART {}

/// Pin is used by SDIO
pub struct SDIO {}

/// Pin is used by SPI
pub struct SPI {}

/// Pin is used by HSPI
pub struct HSPI {}

/// Pin is used by I2S
pub struct I2S {}

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
       Gpio0: (gpio0, Input<Floating>),
       Gpio1: (gpio1, UART),
       Gpio2: (gpio2, Input<Floating>),
       Gpio3: (gpio3, UART),
       Gpio4: (gpio4, Input<Floating>),
       Gpio5: (gpio5, Input<Floating>),
       Gpio6: (gpio6, UnInitialized),
       Gpio7: (gpio7, UnInitialized),
       Gpio8: (gpio8, UnInitialized),
       Gpio9: (gpio9, UnInitialized),
       Gpio10: (gpio10, UnInitialized),
       Gpio11: (gpio11, UnInitialized),
       Gpio12: (gpio12, UnInitialized),
       Gpio13: (gpio13, UART),
       Gpio14: (gpio14, UnInitialized),
       Gpio15: (gpio15, UART),
   ]
}

macro_rules! impl_input_output {
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
                    Ok(!self.is_high()?)
                }
            }

            impl<MODE> StatefulOutputPin for $pxi<Output<MODE>> {
               fn is_set_high(&self) -> Result<bool, Self::Error> {
                   let input = unsafe { (*GPIO::ptr()).$in.read().gpio_in_data().bits() };
                    Ok((input >> $i) & 1 == 1)
               }
               fn is_set_low(&self) -> Result<bool, Self::Error> {
                   Ok(!self.is_set_high()?)
               }
            }

            impl<MODE> toggleable::Default for $pxi<Output<MODE>> {}

            impl<MODE> $pxi<MODE> {
                pub fn into_push_pull_output(self) -> $pxi<Output<PushPull>> {
                    let gpio = unsafe{ &*GPIO::ptr() };
                    let iomux = unsafe{ &*IO_MUX::ptr() };
                    iomux.$iomux.modify(|_, w| unsafe {
                        w.function_select_low_bits().bits($mcu_sel_bits & 0b11)
                        .function_select_high_bit().bit($mcu_sel_bits >> 2 == 1)
                    });

                    gpio.$en.write(|w| unsafe { w.bits(0x1 << $i) });

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

                    gpio.$dis.write(|w| unsafe { w.bits(0x1 << $i) });

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

                    gpio.$dis.write(|w| unsafe { w.bits(0x1 << $i) });

                    $pxi { _mode: PhantomData }
                }
            }
        )+
    };
}

impl_input_output! {
    gpio_enable_w1ts, gpio_enable_w1tc, gpio_out_w1ts, gpio_out_w1tc, gpio_in, [
        Gpio0: (0, pin0, io_mux_gpio0, 0b000),
        Gpio1: (1, pin1, io_mux_u0txd, 0b011),
        Gpio2: (2, pin2, io_mux_gpio2, 0b000),
        Gpio3: (3, pin3, io_mux_u0rxd, 0b011),
        Gpio4: (4, pin4, io_mux_gpio4, 0b000),
        Gpio5: (5, pin5, io_mux_gpio5, 0b000),
        Gpio6: (6, pin6, io_mux_sd_clk, 0b011),
        Gpio7: (7, pin7, io_mux_sd_data0, 0b011),
        Gpio8: (8, pin8, io_mux_sd_data1, 0b011),
        Gpio9: (9, pin9, io_mux_sd_data2, 0b011),
        Gpio10: (10, pin10, io_mux_sd_data3, 0b011),
        Gpio11: (11, pin11, io_mux_sd_cmd, 0b011),
        Gpio12: (12, pin12, io_mux_mtdi, 0b011),
        Gpio13: (13, pin13, io_mux_mtck, 0b011),
        Gpio14: (14, pin14, io_mux_mtms, 0b011),
        Gpio15: (15, pin15, io_mux_mtdo, 0b011),
    ]
}

macro_rules! impl_into_mode {
    ($mode:ident, $func:ident, $dis:ident, [
        // index, gpio pin name, funcX name, iomux pin name, iomux mcu_sel bits
        $($pxi:ident: ($i:expr, $iomux:ident, $mcu_sel_bits:literal, $pull_up:literal),)+
    ]) => {
        $(
            impl<MODE> $pxi<MODE> {
                pub fn $func(self) -> $pxi<$mode> {
                    let gpio = unsafe{ &*GPIO::ptr() };
                    let iomux = unsafe{ &*IO_MUX::ptr() };
                    iomux.$iomux.modify(|_, w| unsafe {
                        w.function_select_low_bits().bits($mcu_sel_bits & 0b11)
                        .function_select_high_bit().bit($mcu_sel_bits >> 2 == 1)
                        .pullup().bit($pull_up)
                    });

                    gpio.$dis.write(|w| unsafe { w.bits(0x1 << $i) });

                    $pxi { _mode: PhantomData }
                }
            }
        )+
    };
}

impl_into_mode! {
    UART, into_uart, gpio_enable_w1tc, [
        Gpio1: (1, io_mux_u0txd, 0b000, false),
        Gpio2: (2, io_mux_gpio2, 0b010, false),
        Gpio3: (3, io_mux_u0rxd, 0b000, false),
        Gpio13: (13, io_mux_mtck, 0b000, false),
        Gpio15: (15, io_mux_mtdo, 0b000, false),
    ]
}

impl_into_mode! {
    SDIO, into_sdio, gpio_enable_w1tc, [
        Gpio6: (6, io_mux_sd_clk, 0b000, false),
        Gpio7: (7, io_mux_sd_data0, 0b000, false),
        Gpio8: (8, io_mux_sd_data1, 0b000, false),
        Gpio9: (9, io_mux_sd_data2, 0b000, false),
        Gpio10: (10, io_mux_sd_data3, 0b000, false),
        Gpio11: (11, io_mux_sd_cmd, 0b000, false),
    ]
}

impl_into_mode! {
    SPI, into_spi, gpio_enable_w1tc, [
        Gpio6: (6, io_mux_sd_clk, 0b001, true),
        Gpio7: (7, io_mux_sd_data0, 0b001, true),
        Gpio8: (8, io_mux_sd_data1, 0b001, true),
        Gpio9: (9, io_mux_sd_data2, 0b001, false),
        Gpio10: (10, io_mux_sd_data3, 0b001, false),
        Gpio1: (1, io_mux_u0txd, 0b001, false),
        Gpio0: (0, io_mux_gpio0, 0b001, false),
    ]
}

impl_into_mode! {
    HSPI, into_hspi, gpio_enable_w1tc, [
        Gpio14: (14, io_mux_mtms, 0b010, true),
        Gpio12: (12, io_mux_mtdi, 0b010, true),
        Gpio13: (13, io_mux_mtck, 0b010, true),
        Gpio15: (15, io_mux_mtdo, 0b010, true),
    ]
}

impl_into_mode! {
    I2S, into_i2s, gpio_enable_w1tc, [
        Gpio12: (12, io_mux_mtdi, 0b001, false),
        Gpio13: (13, io_mux_mtck, 0b001, false),
        Gpio14: (14, io_mux_mtms, 0b001, false),
        Gpio15: (15, io_mux_mtdo, 0b001, false),
        Gpio3: (3, io_mux_u0rxd, 0b001, false),
        Gpio2: (2, io_mux_gpio2, 0b001, false),
    ]
}
