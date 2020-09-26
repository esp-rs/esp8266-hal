#![no_std]
#![no_main]

use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;
use panic_halt as _;
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = dp.GPIO.split();
    let mut serial = dp.UART0.serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());

    let (mut timer1, _) = dp.TIMER.timers();
    timer1.delay_ms(100);

    write!(serial, "\r\nstart:\r\n").unwrap();

    loop {
        let byte = nb::block!(serial.read()).unwrap();
        serial.write(byte).unwrap();
    }
}
