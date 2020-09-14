use core::convert::Infallible;
use core::marker::PhantomData;

use embedded_hal::digital::v2::{toggleable, InputPin, OutputPin, StatefulOutputPin};
use esp8266::{GPIO, IO_MUX, RTC};

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

#[repr(u8)]
pub enum InterruptMode {
    Disabled = 0,
    PositiveEdge = 1,
    NegativeEdge = 2,
    BothEdges = 3,
    LowLevel = 4,
    HighLevel = 5,
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
       Gpio16: (gpio16, Input<Floating>),
   ]
}

macro_rules! impl_input_output {
    ($en:ident, $dis:ident, $outs:ident, $outc:ident, $in:ident, $int_status:ident, $int_clear:ident, [
        // index, gpio pin name, funcX name, iomux pin name, iomux mcu_sel bits
        $($pxi:ident: ($i:expr, $pin:ident, $iomux:ident, $mcu_sel_bits:expr, $driver:ident, $int:ident),)+
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

            impl<MODE> $pxi<Input<MODE>> {
                pub fn set_interrupt_mode(&mut self, mode: InterruptMode) {
                    let gpio = unsafe { &*GPIO::ptr() };
                    gpio.$pin.modify(|_, w| unsafe { w.$int().bits(mode as u8) });
                }

                pub fn clear_interrupt(&mut self) {
                    unsafe { (*GPIO::ptr()).$int_clear.write(|w| w.bits(1 << $i)) };
                }

                pub fn interrupt_status(&mut self) -> bool {
                    let status = unsafe { (*GPIO::ptr()).$int_status.read().bits() };
                    status & (1 << $i) > 0
                }
            }

            impl InputPin for $pxi<Output<OpenDrain>> {
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
                unsafe fn configure_gpio(&mut self, mode: u8, enable: bool, pullup: bool, open_drain: bool) {
                    let gpio = &*GPIO::ptr();
                    let iomux = &*IO_MUX::ptr();
                    iomux.$iomux.modify(|_, w| {
                        w.function_select_low_bits().bits(mode & 0b11)
                        .function_select_high_bit().bit(mode >> 2 == 1)
                        .pullup().bit(pullup)
                    });

                    if enable {
                        gpio.$en.write(|w| w.bits(0x1 << $i));
                    } else {
                        gpio.$dis.write(|w| w.bits(0x1 << $i));
                    }

                    gpio.$pin.modify(|_, w| w.$driver().bit(open_drain));
                }

                /// Change the pin into a specific mode without doing any of the pin configuration
                ///
                /// This should only be used if you know the pin is already configured correctly
                pub unsafe fn unsafe_info<T>(self) -> $pxi<T> {
                    $pxi { _mode: PhantomData }
                }

                pub fn into_push_pull_output(mut self) -> $pxi<Output<PushPull>> {
                    unsafe { self.configure_gpio($mcu_sel_bits, true, false, false) };

                    $pxi { _mode: PhantomData }
                }
                pub fn into_open_drain_output(mut self) -> $pxi<Output<OpenDrain>> {
                    unsafe { self.configure_gpio($mcu_sel_bits, true, false, true) };

                    $pxi { _mode: PhantomData }
                }

                pub fn into_floating_input(mut self) -> $pxi<Input<Floating>> {
                    unsafe { self.configure_gpio($mcu_sel_bits, false, false, false) };

                    $pxi { _mode: PhantomData }
                }

                pub fn into_pull_up_input(mut self) -> $pxi<Input<PullUp>> {
                    unsafe { self.configure_gpio($mcu_sel_bits, false, true, false) };

                    $pxi { _mode: PhantomData }
                }
            }
        )+
    };
}

impl_input_output! {
    gpio_enable_w1ts, gpio_enable_w1tc, gpio_out_w1ts, gpio_out_w1tc, gpio_in, gpio_status, gpio_status_w1tc, [
        Gpio0: (0, gpio_pin0, io_mux_gpio0, 0b000, gpio_pin0_driver, gpio_pin0_int_type),
        Gpio1: (1, gpio_pin1, io_mux_u0txd, 0b011, gpio_pin1_driver, gpio_pin1_int_type),
        Gpio2: (2, gpio_pin2, io_mux_gpio2, 0b000, gpio_pin2_driver, gpio_pin2_int_type),
        Gpio3: (3, gpio_pin3, io_mux_u0rxd, 0b011, gpio_pin3_driver, gpio_pin3_int_type),
        Gpio4: (4, gpio_pin4, io_mux_gpio4, 0b000, gpio_pin4_driver, gpio_pin4_int_type),
        Gpio5: (5, gpio_pin5, io_mux_gpio5, 0b000, gpio_pin5_driver, gpio_pin5_int_type),
        Gpio6: (6, gpio_pin6, io_mux_sd_clk, 0b011, gpio_pin6_driver, gpio_pin6_int_type),
        Gpio7: (7, gpio_pin7, io_mux_sd_data0, 0b011, gpio_pin7_driver, gpio_pin7_int_type),
        Gpio8: (8, gpio_pin8, io_mux_sd_data1, 0b011, gpio_pin8_driver, gpio_pin8_int_type),
        Gpio9: (9, gpio_pin9, io_mux_sd_data2, 0b011, gpio_pin9_driver, gpio_pin9_int_type),
        Gpio10: (10, gpio_pin10, io_mux_sd_data3, 0b011, gpio_pin10_driver, gpio_pin10_int_type),
        Gpio11: (11, gpio_pin11, io_mux_sd_cmd, 0b011, gpio_pin11_driver, gpio_pin11_int_type),
        Gpio12: (12, gpio_pin12, io_mux_mtdi, 0b011, gpio_pin12_driver, gpio_pin12_int_type),
        Gpio13: (13, gpio_pin13, io_mux_mtck, 0b011, gpio_pin13_driver, gpio_pin13_int_type),
        Gpio14: (14, gpio_pin14, io_mux_mtms, 0b011, gpio_pin14_driver, gpio_pin14_int_type),
        Gpio15: (15, gpio_pin15, io_mux_mtdo, 0b011, gpio_pin15_driver, gpio_pin15_int_type),
    ]
}

// GPIO16 is controlled by the RTC instead of the IOMUX, and supports limited
// configurations. Because of this, we need to implement the relevant GPIO
// traits specifically for this pin.
//
// The NodeMCU documentation states that GPIO16 cannot be used as an open-drain
// output or for interrupts, so that functionality has been omitted.
// https://nodemcu.readthedocs.io/en/dev/modules/gpio/

impl<MODE> InputPin for Gpio16<Input<MODE>> {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        let input = unsafe { (*RTC::ptr()).rtc_gpio_in_data.read().bits() };
        Ok(input & 0x1 != 0)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_high()?)
    }
}

impl<PushPull> OutputPin for Gpio16<Output<PushPull>> {
    type Error = Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*RTC::ptr())
                .rtc_gpio_out
                .modify(|r, w| w.bits(r.bits() | 0x1))
        };
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*RTC::ptr())
                .rtc_gpio_out
                .modify(|r, w| w.bits(r.bits() & !0x1))
        };
        Ok(())
    }
}

impl<PushPull> StatefulOutputPin for Gpio16<Output<PushPull>> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        let input = unsafe { (*RTC::ptr()).rtc_gpio_out.read().bits() };
        Ok(input & 0x1 != 0)
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_set_high()?)
    }
}

impl<PushPull> toggleable::Default for Gpio16<Output<PushPull>> {}

impl<MODE> Gpio16<MODE> {
    unsafe fn configure_gpio(&mut self, output: bool, pulldown: bool) {
        let rtc = &(*RTC::ptr());

        // mux configuration for XPD_DCDC and rtc_gpio0 connection
        rtc.pad_xpd_dcdc_conf.modify(|r, w| {
            if pulldown {
                w.bits(r.bits() & 0xffffffbc)
            } else {
                w.bits((r.bits() & 0xffffffbc) | 0x1)
            }
        });

        // mux configuration for out enable
        rtc.rtc_gpio_conf.modify(|r, w| w.bits(r.bits() & !0x1));

        // output enable/disable
        rtc.rtc_gpio_enable.modify(|r, w| {
            if output {
                w.bits(r.bits() | 0x1)
            } else {
                w.bits(r.bits() & !0x1)
            }
        });
    }

    /// Change the pin into a specific mode without doing any of the pin
    /// configuration
    ///
    /// This should only be used if you know the pin is already configured
    /// correctly
    pub unsafe fn unsafe_info<T>(self) -> Gpio16<T> {
        Gpio16 { _mode: PhantomData }
    }

    pub fn into_push_pull_output(mut self) -> Gpio16<Output<PushPull>> {
        unsafe { self.configure_gpio(true, false) };

        Gpio16 { _mode: PhantomData }
    }

    pub fn into_floating_input(mut self) -> Gpio16<Input<PullDown>> {
        unsafe { self.configure_gpio(false, false) };

        Gpio16 { _mode: PhantomData }
    }

    pub fn into_pull_down_input(mut self) -> Gpio16<Input<PullDown>> {
        unsafe { self.configure_gpio(false, true) };

        Gpio16 { _mode: PhantomData }
    }
}

macro_rules! impl_into_mode {
    ($mode:ident, $func:ident, $dis:ident, [
        // index, gpio pin name, funcX name, iomux pin name, iomux mcu_sel bits
        $($pxi:ident: ($i:expr, $iomux:ident, $mcu_sel_bits:literal, $pull_up:literal),)+
    ]) => {
        $(
            impl<MODE> $pxi<MODE> {
                pub fn $func(mut self) -> $pxi<$mode> {
                    unsafe { self.configure_gpio($mcu_sel_bits, false, $pull_up, false) };

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
        Gpio12: (12, io_mux_mtdi, 0b010, false),
        Gpio13: (13, io_mux_mtck, 0b010, false),
        Gpio14: (14, io_mux_mtms, 0b010, false),
        Gpio15: (15, io_mux_mtdo, 0b010, false),
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
