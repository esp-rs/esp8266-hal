#![no_std]
#![no_main]

use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let dp = unsafe { Peripherals::steal() };
    let pins = dp.GPIO.split();
    let serial = dp.UART0.serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());
    let mut led = pins.gpio2.into_push_pull_output();
    let (mut timer1, _) = dp.TIMER.timers();

    let _handler = serial.attach_interrupt(|serial| {
        if let Ok(byte) = serial.read() {
            let _ = serial.write(byte);
        }
    });

    led.set_high().unwrap();

    loop {
        timer1.delay_ms(500);
        led.toggle().unwrap();
    }
}
