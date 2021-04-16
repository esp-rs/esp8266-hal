//! UART peripheral control.
//!
//! Implements the traits required for reading from and writing to a serial port.
//!
//! The ESP8266 has two UARTs available, `UART0` and `UART1`. `UART1` is write-only.

use crate::gpio::*;

use embedded_hal::serial::{Read, Write};
use esp8266::{UART0, UART1};
use void::Void;

/// Extension trait for `UART0` for easily creating `UART0Serial` instances.
pub trait UART0Ext {
    fn serial(self, tdx: Gpio1<UART>, rxd: Gpio3<UART>) -> UART0Serial;
}

impl UART0Ext for UART0 {
    /// Create a new `UART0Serial` instance using the provided pins.
    fn serial(self, txd: Gpio1<UART>, rxd: Gpio3<UART>) -> UART0Serial {
        UART0Serial {
            uart: self,
            txd,
            rxd,
        }
    }
}

/// Extension trait for `UART1` for easily creating `UART1Serial` instances.
///
/// Note that `UART1` is write-only.
pub trait UART1Ext {
    fn serial(self, txd: Gpio2<UART>) -> UART1Serial;
}

impl UART1Ext for UART1 {
    /// Create a new write-only `UART1Serial` instance using the provided pin.
    fn serial(self, txd: Gpio2<UART>) -> UART1Serial {
        UART1Serial { uart: self, txd }
    }
}

/// Serial port using `UART0`.
pub struct UART0Serial {
    uart: UART0,
    txd: Gpio1<UART>,
    rxd: Gpio3<UART>,
}

impl UART0Serial {
    /// Free up the UART device and return the pins used.
    ///
    /// This operation blocks while there are still bytes in the transmit buffer.
    pub fn decompose(mut self) -> nb::Result<(UART0, Gpio1<UART>, Gpio3<UART>), Void> {
        self.flush()?;
        Ok((self.uart, self.txd, self.rxd))
    }
}

impl Read<u8> for UART0Serial {
    type Error = Void;

    /// Reads a single word from the serial interface.
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        if self.uart.uart_status.read().rxfifo_cnt().bits() > 0 {
            Ok(self.uart.uart_fifo.read().rxfifo_rd_byte().bits())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl Write<u8> for UART0Serial {
    type Error = Void;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if self.uart.uart_status.read().txfifo_cnt().bits() < 128 {
            self.uart
                .uart_fifo
                .write(|w| unsafe { w.rxfifo_write_byte().bits(word) });
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn flush(&mut self) -> nb::Result<(), Void> {
        if self.uart.uart_status.read().txfifo_cnt().bits() > 0 {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}

impl core::fmt::Write for UART0Serial {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        s.as_bytes()
            .iter()
            .try_for_each(|c| nb::block!(self.write(*c)))
            .map_err(|_| core::fmt::Error)
    }
}

/// Write-only serial port using `UART1`.
pub struct UART1Serial {
    uart: UART1,
    txd: Gpio2<UART>,
}

impl UART1Serial {
    /// Free up the UART device and return the pins used.
    ///
    /// This operation blocks while there are still bytes in the transmit buffer.
    pub fn decompose(mut self) -> nb::Result<(UART1, Gpio2<UART>), Void> {
        self.flush()?;
        Ok((self.uart, self.txd))
    }
}

impl Write<u8> for UART1Serial {
    type Error = Void;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if self.uart.uart_status.read().txfifo_cnt().bits() < 128 {
            self.uart
                .uart_fifo
                .write(|w| unsafe { w.rxfifo_write_byte().bits(word) });
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn flush(&mut self) -> nb::Result<(), Void> {
        if self.uart.uart_status.read().txfifo_cnt().bits() > 0 {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}

impl core::fmt::Write for UART1Serial {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        s.as_bytes()
            .iter()
            .try_for_each(|c| nb::block!(self.write(*c)))
            .map_err(|_| core::fmt::Error)
    }
}

impl embedded_hal::blocking::serial::write::Default<u8> for UART0Serial {}

impl embedded_hal::blocking::serial::write::Default<u8> for UART1Serial {}

#[cfg(all(feature = "rt", feature = "interrupt"))]
mod interrupt {
    use super::*;

    int_handler!(UART => UartInterruptHandler(UART0Serial));

    impl UART0Serial {
        #[must_use = "the interrupt handler must be kept in scope for the interrupt to be handled"]
        pub fn attach_interrupt<F: FnMut(&mut UART0Serial)>(
            self,
            mut f: F,
        ) -> Pin<UartInterruptHandler<impl FnMut(&mut UART0Serial)>> {
            UartInterruptHandler::new(self, move |serial: &mut UART0Serial| {
                f(serial);
                serial.uart.uart_int_clr.write(|w| unsafe { w.bits(0xff) });
            })
        }
    }
}
