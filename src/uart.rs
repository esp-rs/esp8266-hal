use embedded_hal::serial::{Read, Write};
use esp8266::UART;
use void::Void;

pub trait UARTExt {
    fn serial(self) -> Serial;
}

impl UARTExt for UART {
    fn serial(self) -> Serial {
        Serial { uart: self }
    }
}

pub struct Serial {
    uart: UART,
}

impl Write<u8> for Serial {
    type Error = Void;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.uart
            .uart_fifo
            .write(|w| unsafe { w.bits(word as u32) });
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // noop
        Ok(())
    }
}

impl Read<u8> for Serial {
    /// Read error
    type Error = Void;

    /// Reads a single word from the serial interface
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let state = self.uart.uart_status.read();
        if state.uart_rxfifo_cnt().bits() > 0 {
            Ok(self.uart.uart_fifo.read().uart_rxfifo_rd_byte().bits())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
