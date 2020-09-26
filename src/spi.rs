use crate::gpio::*;
use embedded_hal::spi::{FullDuplex, Mode, Phase, Polarity};
use esp8266::{IO_MUX, SPI1};
use void::Void;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum SpiClock {
    Spi2MHz = 40,
    Spi4MHz = 20,
    Spi5MHz = 16,
    Spi8MHz = 10,
    Spi10MHz = 8,
    Spi16MHz = 5,
    Spi20MHz = 4,
    Spi40MHz = 2,
    Spi80MHz = 1,
}

pub trait SPIExt {
    fn spi(
        self,
        sclk: Gpio14<HSPI>,
        miso: Gpio12<HSPI>,
        mosi: Gpio13<HSPI>,
        frequency: SpiClock,
    ) -> SPI1Master;
}

impl SPIExt for SPI1 {
    fn spi(
        self,
        sclk: Gpio14<HSPI>,
        miso: Gpio12<HSPI>,
        mosi: Gpio13<HSPI>,
        frequency: SpiClock,
    ) -> SPI1Master {
        SPI1Master::new(self, sclk, miso, mosi, frequency)
    }
}

pub struct SPI1Master {
    spi: SPI1,
    sclk: Gpio14<HSPI>,
    miso: Gpio12<HSPI>,
    mosi: Gpio13<HSPI>,
}

impl SPI1Master {
    pub fn new(
        spi: SPI1,
        sclk: Gpio14<HSPI>,
        miso: Gpio12<HSPI>,
        mosi: Gpio13<HSPI>,
        frequency: SpiClock,
    ) -> Self {
        let mut spi = SPI1Master {
            spi,
            sclk,
            miso: miso.into_push_pull_output().into_hspi(),
            mosi,
        };

        spi.spi.spi_ctrl.write_with_zero(|w| w);
        spi.set_frequency(frequency);
        spi.spi.spi_user.write(|w| {
            w.spi_usr_mosi()
                .set_bit()
                .spi_duplex()
                .set_bit()
                .spi_ck_i_edge()
                .set_bit()
        });
        spi.set_data_bits(8);
        spi.spi.spi_ctrl1.write_with_zero(|w| w);

        spi
    }

    /// free up the spi device and return the pins used
    pub fn decompose(self) -> nb::Result<(SPI1, Gpio14<HSPI>, Gpio12<HSPI>, Gpio13<HSPI>), Void> {
        Ok((self.spi, self.sclk, self.miso, self.mosi))
    }

    pub fn set_data_mode(&mut self, mode: Mode) {
        match mode.phase {
            Phase::CaptureOnFirstTransition => self
                .spi
                .spi_user
                .modify(|_, w| w.spi_ck_o_edge().clear_bit()),
            Phase::CaptureOnSecondTransition => {
                self.spi.spi_user.modify(|_, w| w.spi_ck_o_edge().set_bit())
            }
        }

        match mode.polarity {
            Polarity::IdleLow => self
                .spi
                .spi_pin
                .modify(|_, w| w.spi_idle_edge().clear_bit()),
            Polarity::IdleHigh => self.spi.spi_pin.modify(|_, w| w.spi_idle_edge().set_bit()),
        }
    }

    pub fn set_frequency(&mut self, frequency: SpiClock) {
        let iomux = unsafe { &*IO_MUX::ptr() };
        if frequency == SpiClock::Spi80MHz {
            iomux
                .io_mux_conf
                .modify(|_, w| w.spi1_clk_equ_sys_clk().set_bit());
            self.spi
                .spi_clock
                .write_with_zero(|w| w.spi_clk_equ_sysclk().set_bit());
        } else {
            iomux
                .io_mux_conf
                .modify(|_, w| w.spi1_clk_equ_sys_clk().clear_bit());
            self.spi.spi_clock.write_with_zero(|w| unsafe {
                w.spi_clkcnt_n()
                    .bits(frequency as u8 - 1)
                    .spi_clkcnt_h()
                    .bits((frequency as u8) / 2 - 1)
                    .spi_clkcnt_l()
                    .bits((frequency as u8 - 1) & 0x40)
            });
        }
    }

    fn set_data_bits(&mut self, bits: u16) {
        self.spi.spi_user1.write(|w| unsafe {
            w.reg_usr_mosi_bitlen()
                .bits(bits - 1)
                .reg_usr_miso_bitlen()
                .bits(bits - 1)
        });
    }
}

impl FullDuplex<u8> for SPI1Master {
    type Error = Void;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        if self.spi.spi_cmd.read().spi_usr().bit_is_set() {
            return Err(nb::Error::WouldBlock);
        }

        Ok((self.spi.spi_w0.read().bits() & 0xFF) as u8)
    }

    fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if self.spi.spi_cmd.read().spi_usr().bit_is_set() {
            return Err(nb::Error::WouldBlock);
        }

        self.set_data_bits(8);

        self.spi.spi_w0.write(|w| unsafe { w.bits(word as u32) });
        self.spi.spi_cmd.modify(|_, w| w.spi_usr().set_bit());

        Ok(())
    }
}

impl embedded_hal::blocking::spi::write::Default<u8> for SPI1Master {}

impl embedded_hal::blocking::spi::transfer::Default<u8> for SPI1Master {}
