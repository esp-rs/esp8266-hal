//! The prelude.
//!
//! To use `esp8266_hal` effectively, a number of traits and types need to be
//! imported. Instead of importing them one by one manually, the prelude
//! contains the most commonly used imports that are used around application
//! runtime management.
//!
//! This can be imported as use `esp8266_hal::prelude::*`.

pub use crate::efuse::DPortExt;
pub use crate::flash::SPI0Ext;
pub use crate::gpio::GpioExt;
pub use crate::rng::RngExt;
pub use crate::rtccntl::RtcControlExt;
pub use crate::spi::SPIExt;
pub use crate::time::U32Ext;
pub use crate::timer::TimerExt;
pub use crate::uart::{UART0Ext, UART1Ext};
pub use crate::watchdog::WatchdogExt;
pub use esp8266_hal_proc_macros::{interrupt, ram};
pub use xtensa_lx_rt::{entry, exception};

pub use embedded_hal::digital::v2::InputPin as _;
pub use embedded_hal::digital::v2::OutputPin as _;
pub use embedded_hal::digital::v2::StatefulOutputPin as _;
pub use embedded_hal::digital::v2::ToggleableOutputPin as _;
pub use embedded_hal::prelude::*;
pub use embedded_hal::timer::{Cancel, CountDown, Periodic};
