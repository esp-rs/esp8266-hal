use embedded_hal::timer::{Cancel, CountDown, Periodic};
use esp8266::timer::frc1_ctrl::{INTERRUPT_TYPE_A, PRESCALE_DIVIDER_A};
use esp8266::TIMER;
use void::Void;

#[derive(Clone, Copy)]
pub struct Nanoseconds(pub u32);

pub trait TimerExt {
    fn timer(self, frequency: u32) -> Timer;
}

impl TimerExt for TIMER {
    fn timer(self, frequency: u32) -> Timer {
        Timer::new(self, frequency)
    }
}

pub struct Timer {
    timer: TIMER,
    ticks_per_ms: u32,
}

impl Timer {
    fn new(timer: TIMER, frequency: u32) -> Self {
        timer.frc1_ctrl.write(|w| {
            w.rollover()
                .set_bit()
                .timer_enable()
                .set_bit()
                .interrupt_type()
                .level()
                .prescale_divider()
                .devided_by_256()
        });

        Timer {
            timer,
            ticks_per_ms: (1_000_000_000 / (frequency / 256)),
        }
    }
}

impl CountDown for Timer {
    type Time = Nanoseconds;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Nanoseconds>,
    {
        let timeout: Nanoseconds = timeout.into();

        let ticks = timeout.0 / self.ticks_per_ms;
        self.timer.frc1_load.write(|w| unsafe { w.bits(ticks) });
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        if self.timer.frc1_ctrl.read().frc1_int().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            self.timer
                .frc1_int
                .modify(|_, w| w.frc1_int_clr_mask().set_bit());
            Ok(())
        }
    }
}

impl Periodic for Timer {}

impl Cancel for Timer {
    type Error = Void;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.timer
            .frc1_ctrl
            .modify(|_, w| w.timer_enable().clear_bit());
        Ok(())
    }
}
