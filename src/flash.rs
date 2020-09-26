use crate::ram;
use esp8266::{SPI0};
use void::Void;

#[cfg(not(feature = "all_in_ram"))]
mod cache;

/// dummy implementation of cache control for when we don't need cache enabled
#[cfg(feature = "all_in_ram")]
mod cache {
    use esp8266::SPI0;
    use core::ops::{DerefMut, Deref};

    pub(crate) fn cache_enable(_spi0: &mut SPI0, _mb: u8) {
        // noop
    }

    pub(super) struct CachePauseGuard<'a> {
        spi: &'a mut SPI0
    }

    impl<'a> CachePauseGuard<'a> {
        pub fn new(spi: &'a mut SPI0) -> Self {
            CachePauseGuard { spi }
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
}

use cache::CachePauseGuard;
pub(crate) use cache::{cache_enable};

/// Access for the ESP8266 builtin flash
pub struct ESPFlash {
    spi: SPI0,
}

impl ESPFlash {
    pub fn new(spi: SPI0) -> Self {
        ESPFlash { spi }
    }
}

#[inline]
fn write_enable(spi: &mut SPI0) {
    spi.spi_addr.write(|w| unsafe { w.bits(0) });
    spi.spi_cmd.write(|w| w.spi_write_enable().set_bit());

    while spi.spi_cmd.read().bits() > 0 {}
}

#[inline]
fn get_status(spi: &mut SPI0) -> u32 {
    spi.spi_addr.write(|w| unsafe { w.bits(0) });
    spi.spi_cmd.write(|w| w.spi_read_sr().set_bit());

    while spi.spi_cmd.read().bits() > 0 {}

    spi.spi_rd_status.read().bits()
}

impl ESPFlash {
    /// Erase 4K sectors
    #[ram]
    pub fn erase_sectors(&mut self, addr: u32, sector_count: usize) -> Result<(), Void> {
        let mut spi = CachePauseGuard::new(&mut self.spi);
        write_enable(&mut spi);

        for i in 0..sector_count {
            spi.spi_addr.write(|w| unsafe { w.address().bits(addr + i as u32) });
            spi.spi_cmd.write(|w| w.spi_se().set_bit());

            while spi.spi_cmd.read().bits() > 0 {}

            while get_status(&mut spi) & 1 > 0 {}
        }

        Ok(())
    }

    #[ram]
    pub fn erase_all(&mut self) -> Result<(), Void> {
        let mut spi = CachePauseGuard::new(&mut self.spi);
        write_enable(&mut spi);

        spi.spi_cmd.write(|w| w.spi_ce().set_bit());

        while spi.spi_cmd.read().bits() > 0 {}

        while get_status(&mut spi) & 1 > 0 {}

        Ok(())
    }

    #[ram]
    pub fn write_bytes(&mut self, addr: u32, data: &mut [u8]) -> Result<(), Void> {
        let mut spi = CachePauseGuard::new(&mut self.spi);
        write_enable(&mut spi);

        // todo 64 byte chunks
        for (i, byte) in data.iter().enumerate() {
            spi.spi_addr.write(|w| unsafe { w.address().bits(addr + i as u32).size().bits(1) });
            spi.spi_w0.write(|w| unsafe { w.bits(*byte as u32) });
            spi.spi_cmd.write(|w| w.spi_pp().set_bit());

            while spi.spi_cmd.read().bits() > 0 {}

            while get_status(&mut spi) & 1 > 0 {}
        }

        Ok(())
    }

    #[ram]
    pub fn read(&mut self, addr: u32, buf: &mut [u8]) -> Result<(), Void> {
        let spi = CachePauseGuard::new(&mut self.spi);

        for (i, byte) in buf.iter_mut().enumerate() {
            spi.spi_addr.write(|w| unsafe { w.address().bits(addr + i as u32).size().bits(1) });
            spi.spi_cmd.write(|w| w.spi_read().set_bit());

            while spi.spi_cmd.read().bits() > 0 {}

            *byte = spi.spi_w0.read().bits() as u8;
        }

        Ok(())
    }
}
