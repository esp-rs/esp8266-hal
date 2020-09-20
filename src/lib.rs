#![no_std]

pub use embedded_hal as ehal;
pub use esp8266 as target;
pub use esp8266_hal_proc_macros::{interrupt, ram};

#[cfg(feature = "rt")]
pub use xtensa_lx_rt::entry;

#[cfg(all(feature = "rt", feature = "interrupt"))]
#[macro_use]
mod interrupt;

pub mod gpio;
pub mod prelude;
pub mod rng;
pub mod spi;
pub mod time;
pub mod timer;
pub mod uart;
pub mod watchdog;
pub mod dport;
pub mod rtccntl;

extern "C" {
    // todo: replace this with a rust implementation
    fn Cache_Read_Enable(map: u8, p: u8, v: u8);
}

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
    // setup the flash memory mapping
    Cache_Read_Enable(0, 0, 0);

    // configure the pll for the most common crystal frequency
    use rtccntl::{CrystalFrequency, RtcControlExt};
    use esp8266::Peripherals;
    let dp = Peripherals::steal();
    dp.RTCCNTL.rtc_control().set_crystal_frequency(CrystalFrequency::Crystal26MHz);

    // continue with default reset handler
    xtensa_lx_rt::Reset();
}