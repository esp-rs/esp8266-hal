use crate::time::{Milliseconds, Seconds};
use embedded_hal::watchdog::{Watchdog as WatchdogTrait, WatchdogDisable, WatchdogEnable};
use esp8266::WDT;

// based on the reverse engineering work done by mongoose-os
// https://mongoose-os.com/blog/esp8266-watchdog-timer/

const WATCHDOG_RESET_MAGIC: u32 = 0x73;

pub trait WatchdogExt {
    fn watchdog(self) -> Watchdog;
}

impl WatchdogExt for WDT {
    fn watchdog(self) -> Watchdog {
        Watchdog { watchdog: self }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(u8)]
pub enum StageTimeout {
    StageDisabled = 0,
    /// Roughly 0.84 seconds at 80Mhz
    Stage084Sec = 10,
    /// Roughly 1.68 seconds at 80Mhz
    Stage168Sec = 11,
    /// Roughly 3.36 seconds at 80Mhz
    Stage336Sec = 12,
    /// Roughly 6.71 seconds at 80Mhz
    Stage671Sec = 13,
    /// Roughly 13.4 seconds at 80Mhz
    Stage1342Sec = 14,
    /// Roughly 26.8 seconds at 80Mhz
    Stage2684Sec = 15,
}

impl From<StageTimeout> for Milliseconds {
    fn from(timeout: StageTimeout) -> Milliseconds {
        match timeout {
            StageTimeout::StageDisabled => Milliseconds(0),
            StageTimeout::Stage084Sec => Milliseconds(840),
            StageTimeout::Stage168Sec => Milliseconds(1_680),
            StageTimeout::Stage336Sec => Milliseconds(3_360),
            StageTimeout::Stage671Sec => Milliseconds(6_710),
            StageTimeout::Stage1342Sec => Milliseconds(13_420),
            StageTimeout::Stage2684Sec => Milliseconds(26_840),
        }
    }
}

impl From<Milliseconds> for StageTimeout {
    fn from(ms: Milliseconds) -> StageTimeout {
        StageTimeout::from_upper_bound(ms)
    }
}

impl StageTimeout {
    /// get the highest timeout that is less or equal to the provided time
    fn from_lower_bound(ms: Milliseconds) -> StageTimeout {
        if ms == StageTimeout::StageDisabled.into() {
            StageTimeout::StageDisabled
        } else if ms <= StageTimeout::Stage168Sec.into() {
            StageTimeout::Stage084Sec
        } else if ms <= StageTimeout::Stage336Sec.into() {
            StageTimeout::Stage168Sec
        } else if ms <= StageTimeout::Stage671Sec.into() {
            StageTimeout::Stage336Sec
        } else if ms <= StageTimeout::Stage1342Sec.into() {
            StageTimeout::Stage671Sec
        } else if ms <= StageTimeout::Stage2684Sec.into() {
            StageTimeout::Stage1342Sec
        } else {
            StageTimeout::Stage2684Sec
        }
    }

    /// get the lowest timeout that is more or equal to the provided time
    fn from_upper_bound(ms: Milliseconds) -> StageTimeout {
        if ms == StageTimeout::StageDisabled.into() {
            StageTimeout::StageDisabled
        } else if ms <= StageTimeout::Stage084Sec.into() {
            StageTimeout::Stage084Sec
        } else if ms <= StageTimeout::Stage168Sec.into() {
            StageTimeout::Stage168Sec
        } else if ms <= StageTimeout::Stage336Sec.into() {
            StageTimeout::Stage336Sec
        } else if ms <= StageTimeout::Stage671Sec.into() {
            StageTimeout::Stage671Sec
        } else if ms <= StageTimeout::Stage1342Sec.into() {
            StageTimeout::Stage1342Sec
        } else {
            StageTimeout::Stage2684Sec
        }
    }
}

impl From<StageTimeout> for (StageTimeout, StageTimeout) {
    fn from(stage0: StageTimeout) -> (StageTimeout, StageTimeout) {
        (stage0, StageTimeout::StageDisabled)
    }
}

impl From<Milliseconds> for (StageTimeout, StageTimeout) {
    fn from(ms: Milliseconds) -> (StageTimeout, StageTimeout) {
        let stage0 = StageTimeout::from_lower_bound(ms);
        let stage1_ms = Milliseconds(ms.0.saturating_sub(Milliseconds::from(stage0).0));
        let stage1 = StageTimeout::from_upper_bound(stage1_ms);

        (stage0, stage1)
    }
}

impl From<Seconds> for (StageTimeout, StageTimeout) {
    fn from(s: Seconds) -> (StageTimeout, StageTimeout) {
        Self::from(Milliseconds::from(s))
    }
}

#[test]
fn test_time_into_timeout() {
    use crate::time::U32Ext;

    assert_eq!(
        (StageTimeout::StageDisabled, StageTimeout::StageDisabled),
        0.ms().into()
    );
    assert_eq!(
        (StageTimeout::Stage084Sec, StageTimeout::StageDisabled),
        500.ms().into()
    );
    assert_eq!(
        (StageTimeout::Stage084Sec, StageTimeout::Stage084Sec),
        1500.ms().into()
    );
    assert_eq!(
        (StageTimeout::Stage168Sec, StageTimeout::Stage084Sec),
        2500.ms().into()
    );
    assert_eq!(
        (StageTimeout::Stage2684Sec, StageTimeout::Stage084Sec),
        27000.ms().into()
    );
    assert_eq!(
        (StageTimeout::Stage2684Sec, StageTimeout::Stage2684Sec),
        50000.ms().into()
    );
    assert_eq!(
        (StageTimeout::Stage2684Sec, StageTimeout::Stage2684Sec),
        100000.ms().into()
    );
}

#[test]
fn test_time_into_timeout_is_higher() {
    use crate::time::U32Ext;

    for period in (1..53680).step_by(100) {
        let (stage0, stage1) = period.ms().into();

        let sum_ms = Milliseconds::from(stage0).0 + Milliseconds::from(stage1).0;

        assert!(
            sum_ms >= period,
            "converted period {} is not more than input time {}. stages: {:?}, {:?}",
            sum_ms,
            period,
            stage0,
            stage1
        );
    }
}

pub struct Watchdog {
    watchdog: WDT,
}

impl WatchdogTrait for Watchdog {
    fn feed(&mut self) {
        self.watchdog
            .wdt_rst
            .write(|w| unsafe { w.bits(WATCHDOG_RESET_MAGIC) })
    }
}

impl WatchdogEnable for Watchdog {
    type Time = (StageTimeout, StageTimeout);

    /// Start the watchdog timer
    ///
    /// You can provide the time period either as a value in [`Milliseconds`] or [`Seconds`], a single `StageTimeOut` or a pair of `StageTimeout`
    ///
    /// If the period is provided as a time unit, the actual period will be an approximation of the provided period that is
    /// at least as long as the provided period or the maximum period of 53680 milliseconds.
    ///
    /// Note that the default clock frequency of 80Mhz is assumed when converting from time units.
    ///
    /// [`Milliseconds`]: ../time/struct.Milliseconds.html
    /// [`Seconds`]: ../time/struct.Seconds.html
    fn start<T>(&mut self, period: T)
    where
        T: Into<Self::Time>,
    {
        let (period0, period1) = period.into();
        if period0 == StageTimeout::StageDisabled {
            return;
        }

        self.watchdog.wdt_ctl.write(|w| {
            w.unknown_3()
                .set_bit()
                .unknown_4()
                .set_bit()
                .unknown_5()
                .set_bit()
        });

        self.watchdog
            .wdt_op
            .write(|w| unsafe { w.bits(period0 as u8 as u32) });

        if period1 == StageTimeout::StageDisabled {
            self.watchdog
                .wdt_ctl
                .modify(|_, w| w.stage_1_disable().set_bit());
        } else {
            self.watchdog
                .wdt_op_nd
                .write(|w| unsafe { w.bits(period1 as u8 as u32) });
        }

        self.feed();

        self.watchdog.wdt_ctl.modify(|_, w| w.enable().set_bit());
    }
}

impl WatchdogDisable for Watchdog {
    fn disable(&mut self) {
        self.watchdog.wdt_ctl.modify(|_, w| w.enable().clear_bit());
    }
}
