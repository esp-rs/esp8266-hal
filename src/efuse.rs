use esp8266::EFUSE;

pub trait DPortExt {
    fn read_chip_id(&self) -> u32;
    fn read_mac_addr(&self) -> [u8; 6];
}

impl DPortExt for EFUSE {
    fn read_chip_id(&self) -> u32 {
        let (id0, id1) = (
            self.efuse_data0.read().bits(),
            self.efuse_data1.read().bits(),
        );
        (id0 >> 24) | (id1 << 8)
    }

    fn read_mac_addr(&self) -> [u8; 6] {
        let (mac0, mac1, mac2) = (
            self.efuse_data0.read().bits(),
            self.efuse_data1.read().bits(),
            self.efuse_data3.read().bits(),
        );
        let oui = if mac2 != 0 {
            ((mac2 >> 16) as u8, (mac2 >> 8) as u8, mac2 as u8)
        } else if ((mac1 >> 16) & 0xff) == 0 {
            (0x18, 0xfe, 0x34)
        } else if ((mac1 >> 16) & 0xff) == 1 {
            (0xac, 0xd0, 0x74)
        } else {
            return [0; 6];
        };
        [
            oui.0,
            oui.1,
            oui.2,
            (mac1 >> 8) as u8,
            mac1 as u8,
            (mac0 >> 24) as u8,
        ]
    }
}
