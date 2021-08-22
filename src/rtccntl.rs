use esp8266::RTCCNTL;

pub trait RtcControlExt {
    fn rtc_control(self) -> RtcControl;
}

impl RtcControlExt for RTCCNTL {
    fn rtc_control(self) -> RtcControl {
        RtcControl { rtc_control: self }
    }
}

// Addresses for internal I2C bus for PLL
const I2C_BLOCK: u8 = 0x67;

// PLL config options for common crystal frequencies
const CRYSTAL_12MHZ: (u8, u8) = (0x88, 0xA1);
const CRYSTAL_24MHZ: (u8, u8) = (0x88, 0x71);
const CRYSTAL_26MHZ: (u8, u8) = (0x88, 0x91);
const CRYSTAL_40MHZ: (u8, u8) = (0x08, 0x81);

pub enum CrystalFrequency {
    Crystal12MHz,
    Crystal24MHz,
    Crystal26MHz,
    Crystal40MHz,
}

pub struct RtcControl {
    rtc_control: RTCCNTL,
}

impl RtcControl {
    /// write to internal I2C PLL bus
    fn write_pll_i2c(&mut self, address: u8, data: u8) {
        self.rtc_control.pll.write(|w| unsafe {
            w.block()
                .bits(I2C_BLOCK)
                .addr()
                .bits(address)
                .data()
                .bits(data)
                .write()
                .set_bit()
        });

        while self.rtc_control.pll.read().busy().bit_is_set() {}
    }

    /// Configure the internal PLL for common crystal frequencies
    pub fn set_crystal_frequency(&mut self, crystal: CrystalFrequency) {
        match crystal {
            CrystalFrequency::Crystal12MHz => {
                self.write_pll_i2c(1, CRYSTAL_12MHZ.0);
                self.write_pll_i2c(2, CRYSTAL_12MHZ.1);
            }
            CrystalFrequency::Crystal24MHz => {
                self.write_pll_i2c(1, CRYSTAL_24MHZ.0);
                self.write_pll_i2c(2, CRYSTAL_24MHZ.1);
            }
            CrystalFrequency::Crystal26MHz => {
                self.write_pll_i2c(1, CRYSTAL_26MHZ.0);
                self.write_pll_i2c(2, CRYSTAL_26MHZ.1);
            }
            CrystalFrequency::Crystal40MHz => {
                self.write_pll_i2c(1, CRYSTAL_40MHZ.0);
                self.write_pll_i2c(2, CRYSTAL_40MHZ.1);
            }
        }
    }
}
