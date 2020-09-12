#![no_std]
#![no_main]

use core::panic::PanicInfo;
use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;

#[entry]
fn main() -> ! {
    let dp = unsafe { Peripherals::steal() };
    let pins = dp.GPIO.split();
    let mut serial = dp.UART0.serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());

    loop {
        let byte = nb::block!(serial.read()).unwrap();
        serial.write(byte).unwrap();
    }
}

/// Basic panic handler - just loops
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
