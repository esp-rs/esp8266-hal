#![no_std]

pub use embedded_hal as ehal;
pub use esp8266 as target;
pub use esp8266_hal_proc_macros::{interrupt, ram};

#[cfg(feature = "rt")]
pub use xtensa_lx_rt::{entry, exception};

#[cfg(all(feature = "rt", feature = "interrupt"))]
#[macro_use]
pub mod interrupt;

pub mod efuse;
pub mod flash;
pub mod gpio;
pub mod prelude;
pub mod rng;
pub mod rtccntl;
pub mod spi;
pub mod time;
pub mod timer;
pub mod uart;
pub mod watchdog;

/// Function handling ESP8266 specific initialization
/// then calls original Reset function
///
/// ENTRY point is defined in memory.x
/// *Note: the pre_init function is called in the original reset handler
/// after the initializations done in this function*
#[cfg(feature = "rt")]
#[doc(hidden)]
#[no_mangle]
#[ram]
pub unsafe extern "C" fn ESP8266Reset() -> ! {
    // These symbols come from `memory.x`
    extern "C" {
        static mut _rtc_bss_start: u32;
        static mut _rtc_bss_end: u32;
    }

    // configure the pll for the most common crystal frequency
    use esp8266::Peripherals;
    use rtccntl::{CrystalFrequency, RtcControlExt};
    let mut dp = Peripherals::steal();

    flash::cache_enable(&mut dp.SPI0, 0);

    dp.RTCCNTL
        .rtc_control()
        .set_crystal_frequency(CrystalFrequency::Crystal26MHz);

    // Initialize RTC RAM
    xtensa_lx_rt::zero_bss(&mut _rtc_bss_start, &mut _rtc_bss_end);

    // continue with default reset handler
    xtensa_lx_rt::Reset();
}
