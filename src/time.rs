//! Time units

// Frequency based

/// Bits per second
#[derive(Clone, Copy, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub struct Bps(pub u32);

/// Hertz
#[derive(Clone, Copy, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub struct Hertz(pub u32);

/// KiloHertz
#[derive(Clone, Copy, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub struct KiloHertz(pub u32);

/// MegaHertz
#[derive(Clone, Copy, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub struct MegaHertz(pub u32);

// Period based

/// Seconds
#[derive(Clone, Copy, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub struct Seconds(pub u32);

/// Milliseconds
#[derive(Clone, Copy, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub struct Milliseconds(pub u32);

/// Microseconds
#[derive(Clone, Copy, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub struct Microseconds(pub u32);

/// Nanoseconds
#[derive(Clone, Copy, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub struct Nanoseconds(pub u32);

/// Extension trait that adds convenience methods to the `u32` type
pub trait U32Ext {
    /// Wrap in `Bps`
    fn bps(self) -> Bps;

    /// Wrap in `Hertz`
    fn hz(self) -> Hertz;

    /// Wrap in `KiloHertz`
    fn khz(self) -> KiloHertz;

    /// Wrap in `MegaHertz`
    fn mhz(self) -> MegaHertz;

    /// Wrap in `Seconds`
    fn s(self) -> Seconds;

    /// Wrap in `Milliseconds`
    fn ms(self) -> Milliseconds;

    /// Wrap in `Microseconds`
    fn us(self) -> Microseconds;

    /// Wrap in `NanoSeconds`
    fn ns(self) -> Nanoseconds;
}

impl U32Ext for u32 {
    // Frequency based

    fn bps(self) -> Bps {
        Bps(self)
    }

    fn hz(self) -> Hertz {
        Hertz(self)
    }

    fn khz(self) -> KiloHertz {
        KiloHertz(self)
    }

    fn mhz(self) -> MegaHertz {
        MegaHertz(self)
    }

    // Period based

    fn s(self) -> Seconds {
        Seconds(self)
    }

    fn ms(self) -> Milliseconds {
        Milliseconds(self)
    }

    fn us(self) -> Microseconds {
        Microseconds(self)
    }

    fn ns(self) -> Nanoseconds {
        Nanoseconds(self)
    }
}

// Frequency based

impl From<KiloHertz> for Hertz {
    fn from(from: KiloHertz) -> Hertz {
        Hertz(from.0 * 1_000)
    }
}

impl From<MegaHertz> for Hertz {
    fn from(from: MegaHertz) -> Hertz {
        Hertz(from.0 * 1_000_000)
    }
}

impl From<MegaHertz> for KiloHertz {
    fn from(from: MegaHertz) -> KiloHertz {
        KiloHertz(from.0 * 1_000)
    }
}

impl From<Hertz> for KiloHertz {
    fn from(from: Hertz) -> KiloHertz {
        KiloHertz(from.0 / 1_000)
    }
}

impl From<Hertz> for MegaHertz {
    fn from(from: Hertz) -> MegaHertz {
        MegaHertz(from.0 / 1_000_000)
    }
}

impl From<KiloHertz> for MegaHertz {
    fn from(from: KiloHertz) -> MegaHertz {
        MegaHertz(from.0 / 1_000)
    }
}

// Period based

impl From<Seconds> for Milliseconds {
    fn from(from: Seconds) -> Milliseconds {
        Milliseconds(from.0 * 1_000)
    }
}
impl From<Seconds> for Microseconds {
    fn from(from: Seconds) -> Microseconds {
        Microseconds(from.0 * 1_000_000)
    }
}

impl From<Seconds> for Nanoseconds {
    fn from(from: Seconds) -> Nanoseconds {
        Nanoseconds(from.0 * 1_000_000_000)
    }
}

impl From<Milliseconds> for Microseconds {
    fn from(from: Milliseconds) -> Microseconds {
        Microseconds(from.0 * 1_000)
    }
}

impl From<Microseconds> for Nanoseconds {
    fn from(from: Microseconds) -> Nanoseconds {
        Nanoseconds(from.0 * 1_000)
    }
}

impl From<Milliseconds> for Seconds {
    fn from(from: Milliseconds) -> Seconds {
        Seconds(from.0 / 1_000)
    }
}

impl From<Microseconds> for Seconds {
    fn from(from: Microseconds) -> Seconds {
        Seconds(from.0 / 1_000_000)
    }
}

impl From<Microseconds> for Milliseconds {
    fn from(from: Microseconds) -> Milliseconds {
        Milliseconds(from.0 / 1_000)
    }
}

impl From<Milliseconds> for Nanoseconds {
    fn from(from: Milliseconds) -> Nanoseconds {
        Nanoseconds(from.0 * 1_000_000)
    }
}

// Frequency <-> Period

impl From<Nanoseconds> for Hertz {
    fn from(from: Nanoseconds) -> Hertz {
        Hertz(1_000_000_000_u32 / from.0)
    }
}

impl From<Microseconds> for Hertz {
    fn from(from: Microseconds) -> Hertz {
        Hertz(1_000_000_u32 / from.0)
    }
}

impl From<Nanoseconds> for KiloHertz {
    fn from(from: Nanoseconds) -> KiloHertz {
        KiloHertz(1_000_000_u32 / from.0)
    }
}

impl From<Nanoseconds> for MegaHertz {
    fn from(from: Nanoseconds) -> MegaHertz {
        MegaHertz(1_000_u32 / from.0)
    }
}

impl From<Hertz> for Microseconds {
    fn from(from: Hertz) -> Microseconds {
        Microseconds(1_000_000_u32 / from.0)
    }
}

impl From<Hertz> for Nanoseconds {
    fn from(from: Hertz) -> Nanoseconds {
        Nanoseconds(1_000_000_000u32 / from.0)
    }
}

impl From<KiloHertz> for Nanoseconds {
    fn from(from: KiloHertz) -> Nanoseconds {
        Nanoseconds(1_000_000u32 / from.0)
    }
}

impl From<MegaHertz> for Nanoseconds {
    fn from(from: MegaHertz) -> Nanoseconds {
        Nanoseconds(1_000u32 / from.0)
    }
}
