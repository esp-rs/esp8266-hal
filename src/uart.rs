use embedded_hal::serial::{Read, Write};
use esp8266::{UART0, UART1};
use void::Void;

pub trait UARTExt<UART: UARTPeripheral> {
    fn serial(self) -> Serial<UART>;
}

pub trait UARTPeripheral {
    fn write(&mut self, word: u8) -> nb::Result<(), Void>;
    fn flush(&mut self) -> nb::Result<(), Void>;
    fn read(&mut self) -> nb::Result<u8, Void>;
}

macro_rules! uart_impl {
    ($UART:ident) => {
        impl UARTPeripheral for $UART {
            fn write(&mut self, word: u8) -> nb::Result<(), Void> {
                self.uart_fifo.write(|w| unsafe { w.bits(word as u32) });
                Ok(())
            }

            /// Reads a single word from the serial interface
            fn read(&mut self) -> nb::Result<u8, Void> {
                let state = self.uart_status.read();
                if state.rxfifo_cnt().bits() > 0 {
                    Ok(self.uart_fifo.read().rxfifo_rd_byte().bits())
                } else {
                    Err(nb::Error::WouldBlock)
                }
            }

            fn flush(&mut self) -> nb::Result<(), Void> {
                // noop
                Ok(())
            }
        }

        impl UARTExt<$UART> for $UART {
            fn serial(self) -> Serial<$UART> {
                Serial { uart: self }
            }
        }
    };
}

uart_impl!(UART0);
uart_impl!(UART1);

pub struct Serial<UART: UARTPeripheral> {
    uart: UART,
}

impl<UART: UARTPeripheral> Write<u8> for Serial<UART> {
    type Error = Void;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.uart.write(word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.uart.flush()
    }
}

impl<UART: UARTPeripheral> Read<u8> for Serial<UART> {
    /// Read error
    type Error = Void;

    /// Reads a single word from the serial interface
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.uart.read()
    }
}
