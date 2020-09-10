#![no_std]

pub use embedded_hal as ehal;
pub use esp8266 as target;

#[cfg(feature = "rt")]
pub use xtensa_lx106_rt::entry;

pub mod gpio;
pub mod prelude;
pub mod rng;
pub mod spi;
pub mod time;
pub mod timer;
pub mod uart;
pub mod watchdog;
