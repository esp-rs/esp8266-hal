#![no_std]

pub use embedded_hal as ehal;
pub use esp8266 as target;

pub mod gpio;
pub mod prelude;
pub mod timer;
pub mod uart;
