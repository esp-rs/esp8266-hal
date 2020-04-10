#![no_std]
#![no_main]
#![feature(asm)]

use xtensa_lx6_rt as _;

use core::panic::PanicInfo;
use esp8266_hal as hal;
use esp8266_hal::ehal::digital::v2::OutputPin;
use esp8266_hal::gpio::GpioExt;

/// The default clock source is the onboard crystal
/// In most cases 40mhz (but can be as low as 2mhz depending on the board)
const CORE_HZ: u32 = 40_000_000;

#[no_mangle]
fn main() -> ! {
    let dp = unsafe { hal::pac::Peripherals::steal() };

    // (https://github.com/espressif/openocd-esp8266/blob/97ba3a6bb9eaa898d91df923bbedddfeaaaf28c9/src/target/esp8266.c#L431)
    // openocd disables the wdt's on halt
    // we will do it manually on startup
    let mut timg = dp.TIMER;
    disable_timg_wdts(&mut timg);

    let gpios = dp.GPIO.split();
    let mut led_pin = gpios.gpio2.into_open_drain_output();
    loop {
        led_pin.set_high().unwrap();
        delay(CORE_HZ);
        led_pin.set_low().unwrap();
        delay(CORE_HZ);
    }
}

fn disable_timg_wdts(timg: &mut esp8266::TIMER) {
    timg.frc1_ctrl.write(|w| unsafe { w.bits(0x80) });
    timg.frc2_ctrl.write(|w| unsafe{ w.bits(0x80) });
}

/// cycle accurate delay using the cycle counter register
pub fn delay(clocks: u32) {
    // NOTE: does not account for rollover
    let target = get_ccount() + clocks;
    loop {
        if get_ccount() > target {
            break;
        }
    }
}

/// Performs a special register read to read the current cycle count.
/// In the future, this can be precompiled to a archive (.a) and linked to so we don't
/// have to require the asm nightly feature - see cortex-m-rt for more details
pub fn get_ccount() -> u32 {
    let x: u32;
    unsafe { asm!("rsr.ccount a2" : "={a2}"(x) ) };
    x
}


/// Basic panic handler - just loops
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
