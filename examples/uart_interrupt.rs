#![no_std]
#![no_main]

use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;
use panic_halt as _;
use xtensa_lx::mutex::{CriticalSectionMutex, Mutex};
use esp8266_hal::gpio::{Gpio2, Output, PushPull};

static LED: CriticalSectionMutex<Option<Gpio2<Output<PushPull>>>> = CriticalSectionMutex::new(None);

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = dp.GPIO.split();
    let serial = dp.UART0.serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());
    let mut led = pins.gpio2.into_push_pull_output();
    let (mut timer1, _) = dp.TIMER.timers();

    led.set_high().unwrap();

    (&LED).lock(|led_locked| *led_locked = Some(led));

    // returned handler needs to be assigned to a variable to keep it in scope
    let _handler = serial.attach_interrupt(|serial| {
        if let Ok(byte) = serial.read() {
            let _ = serial.write(byte);
        }
    });

    loop {
        timer1.delay_ms(500);
        (&LED).lock(|led| led.as_mut().unwrap().toggle().unwrap());
    }
}

/// You can both use the attached interrupt callback and set a function as interrupt handler
#[interrupt(uart)]
fn uart() {
    (&LED).lock(|led| led.as_mut().unwrap().toggle().unwrap());
}