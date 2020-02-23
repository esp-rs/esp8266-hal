use embedded_hal::serial::Write;
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
