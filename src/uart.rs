use embedded_hal::serial::{Read, Write};
use esp8266::{UART0, UART1};
use void::Void;

pub trait UART0Ext {
    fn serial(self) -> UART0Serial;
}

impl UART0Ext for UART0 {
    fn serial(self) -> UART0Serial {
        UART0Serial { uart: self }
    }
}

/// UART1 is write only
pub trait UART1Ext {
    fn serial(self) -> UART1Serial;
}

impl UART1Ext for UART1 {
    fn serial(self) -> UART1Serial {
        UART1Serial { uart: self }
    }
}

pub struct UART0Serial {
    uart: UART0,
}

impl Read<u8> for UART0Serial {
    type Error = Void;

    /// Reads a single word from the serial interface
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

pub struct UART1Serial {
    uart: UART1,
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
