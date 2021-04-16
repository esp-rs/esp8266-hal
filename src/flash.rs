use crate::ram;
use esp8266::SPI0;
use void::Void;

pub trait SPI0Ext {
    fn flash(self) -> ESPFlash;
}

impl SPI0Ext for SPI0 {
    fn flash(self) -> ESPFlash {
        ESPFlash { spi: self }
    }
}

#[cfg(not(feature = "all_in_ram"))]
mod cache;

/// dummy implementation of cache control for when we don't need cache enabled
#[cfg(feature = "all_in_ram")]
mod cache {
    use core::ops::{Deref, DerefMut};
    use esp8266::SPI0;

    pub(crate) fn cache_enable(_spi0: &mut SPI0, _mb: u8) {
        // noop
    }

    pub(super) struct CachePauseGuard<'a> {
        spi: &'a mut SPI0,
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

pub(crate) use cache::cache_enable;
use cache::CachePauseGuard;
use core::ptr::{read_volatile, write_volatile};

/// Access for the ESP8266 builtin flash
pub struct ESPFlash {
    spi: SPI0,
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
            spi.spi_addr
                .write(|w| unsafe { w.address().bits(addr + i as u32) });
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
    pub fn write_bytes(&mut self, mut addr: u32, data: &[u8]) -> Result<(), Void> {
        let mut spi = CachePauseGuard::new(&mut self.spi);

        let (head, body, tail) = unsafe { data.align_to::<u32>() };

        addr = write_unaligned(&mut spi, addr, head);

        addr = write_aligned(&mut spi, addr, body);

        write_unaligned(&mut spi, addr, tail);

        Ok(())
    }

    #[ram]
    pub fn read(&mut self, mut addr: u32, buf: &mut [u8]) -> Result<(), Void> {
        let mut spi = CachePauseGuard::new(&mut self.spi);

        let (head, body, tail) = unsafe { buf.align_to_mut::<u32>() };

        addr = read_unaligned(&mut spi, addr, head);

        addr = read_aligned(&mut spi, addr, body);

        read_unaligned(&mut spi, addr, tail);

        Ok(())
    }
}

#[ram]
fn write_unaligned(spi: &mut SPI0, mut addr: u32, data: &[u8]) -> u32 {
    for byte in data {
        write_enable(spi);

        spi.spi_addr
            .write(|w| unsafe { w.address().bits(addr).size().bits(1) });
        spi.spi_w0.write(|w| unsafe { w.bits(*byte as u32) });
        spi.spi_cmd.write(|w| w.spi_pp().set_bit());

        while spi.spi_cmd.read().bits() > 0 {}

        while get_status(spi) & 1 > 0 {}

        addr += 1;
    }

    addr
}

#[ram]
fn write_aligned(spi: &mut SPI0, mut addr: u32, data: &[u32]) -> u32 {
    for chunk in data.chunks(16) {
        write_enable(spi);

        let byte_len = chunk.len() * 4;

        spi.spi_addr
            .write(|w| unsafe { w.address().bits(addr).size().bits(byte_len as u8) });

        let base = spi.spi_w0.as_ptr();
        for (i, num) in chunk.iter().enumerate() {
            unsafe {
                write_volatile(base.add(i), *num);
            }
        }

        spi.spi_cmd.write(|w| w.spi_pp().set_bit());

        while spi.spi_cmd.read().bits() > 0 {}

        while get_status(spi) & 1 > 0 {}

        addr += byte_len as u32;
    }

    addr
}

#[ram]
fn read_unaligned(spi: &mut SPI0, mut addr: u32, data: &mut [u8]) -> u32 {
    for byte in data.iter_mut() {
        spi.spi_addr
            .write(|w| unsafe { w.address().bits(addr).size().bits(1) });
        spi.spi_cmd.write(|w| w.spi_read().set_bit());

        while spi.spi_cmd.read().bits() > 0 {}

        *byte = spi.spi_w0.read().bits() as u8;

        addr += 1;
    }

    addr
}

#[ram]
fn read_aligned(spi: &mut SPI0, mut addr: u32, data: &mut [u32]) -> u32 {
    // the esp8266 hangs when trying to read the full 16 words for unknown reasons
    for chunk in data.chunks_mut(15) {
        let byte_len = chunk.len() * 4;

        spi.spi_addr
            .write(|w| unsafe { w.address().bits(addr).size().bits(byte_len as u8) });
        spi.spi_cmd.write(|w| w.spi_read().set_bit());

        while spi.spi_cmd.read().bits() > 0 {}

        let base = spi.spi_w0.as_ptr();
        for (i, num) in chunk.iter_mut().enumerate() {
            unsafe {
                *num = read_volatile(base.add(i));
            }
        }

        addr += byte_len as u32;
    }

    addr
}
