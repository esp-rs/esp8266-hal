#![no_std]
#![no_main]

use core::fmt::Write;
use esp8266_hal::flash::SPI0Ext;
use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = dp.GPIO.split();
    let mut led = pins.gpio2.into_push_pull_output();
    let (mut timer1, _) = dp.TIMER.timers();
    let mut serial = dp
        .UART0
        .serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());

    led.set_high().unwrap();

    let mut flash = dp.SPI0.flash();

    const ADDR: u32 = 0x7E000;

    timer1.delay_ms(1000);

    let mut buff = [0u8; 8];

    flash.read(ADDR, &mut buff).unwrap();

    buff[0] += 1;

    flash.erase_sectors(ADDR, 1).unwrap();

    flash.write_bytes(ADDR, &buff).unwrap();

    write!(serial, "\r\ncounter {}:\r\n", buff[0]).unwrap();
    write!(serial, "press '0' to reset:\r\n").unwrap();

    loop {
        let byte = nb::block!(serial.read()).unwrap();

        if byte == b'0' {
            buff[0] = 0;

            flash.erase_sectors(ADDR, 1).unwrap();

            flash.write_bytes(ADDR, &mut buff).unwrap();
        }
    }
}
