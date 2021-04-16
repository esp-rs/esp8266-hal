use crate::ram;
use core::ops::{Deref, DerefMut};
use esp8266::{DPORT, SPI0};

#[ram]
pub(crate) fn cache_enable(spi0: &mut SPI0, mb: u8) {
    assert!(mb < 8, "only up to8mb can be mapped");

    let dport = unsafe { &*DPORT::ptr() };
    let offset = mb % 2;
    let block = mb / 2;
    spi0.spi_ctrl.modify(|_, w| w.enable_ahb().set_bit());

    let offset_bits = match offset {
        0 => 0,
        1 => 2,
        _ => 1,
    };
    dport.spi_cache.modify(|_, w| unsafe {
        w.offset()
            .bits(offset_bits)
            .block()
            .bits(block)
            .target()
            .clear_bit()
    });
    dport.spi_cache_target.modify(|_, w| w.target1().set_bit());

    while dport.spi_cache.read().cache_enable().bit_is_clear() {
        // no idea why the sdk sets this in a loop
        dport.spi_cache.modify(|_, w| w.cache_enable().set_bit());
    }
}

#[allow(dead_code)]
pub(crate) fn cache_disable(spi0: &mut SPI0) {
    let dport = unsafe { &*DPORT::ptr() };
    while dport.spi_cache.read().cache_enable().bit_is_set() {
        // no idea why the sdk clears this in a loop
        dport.spi_cache.modify(|_, w| w.cache_enable().clear_bit());
    }
    spi0.spi_ctrl.modify(|_, w| w.enable_ahb().clear_bit());
    dport
        .spi_cache
        .modify(|_, w| w.cache_flush_start().clear_bit());
    dport
        .spi_cache
        .modify(|_, w| w.cache_flush_start().set_bit());
    while dport.spi_cache.read().cache_empty().bit_is_clear() {
        // noop
    }
    dport
        .spi_cache
        .modify(|_, w| w.cache_flush_start().clear_bit());
}

pub(super) struct CachePauseGuard<'a> {
    spi: &'a mut SPI0,
}

impl<'a> CachePauseGuard<'a> {
    /// Stop the spi flash cache until this guard is dropped
    ///
    /// Note that all code called between this creating and dropping this needs to be in iram
    pub fn new(spi: &'a mut SPI0) -> Self {
        let dport = unsafe { &*DPORT::ptr() };
        dport.spi_cache.modify(|_, w| w.cache_enable().clear_bit());

        while spi.spi_ext2.read().bits() != 0 {
            // noop
        }

        spi.spi_ctrl.modify(|_, w| w.enable_ahb().clear_bit());

        CachePauseGuard { spi }
    }
}

impl Drop for CachePauseGuard<'_> {
    #[ram]
    fn drop(&mut self) {
        let dport = unsafe { &*DPORT::ptr() };
        self.spi.spi_ctrl.modify(|_, w| w.enable_ahb().set_bit());
        dport.spi_cache.modify(|_, w| w.cache_enable().set_bit());
    }
}

impl Deref for CachePauseGuard<'_> {
    type Target = SPI0;

    fn deref(&self) -> &Self::Target {
        self.spi
    }
}

impl DerefMut for CachePauseGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.spi
    }
}
