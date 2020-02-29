use embedded_hal::timer::{Cancel, CountDown, Periodic};
use esp8266::timer::frc1_ctrl::{INTERRUPT_TYPE_A, PRESCALE_DIVIDER_A};
use esp8266::TIMER;
use void::Void;

#[derive(Clone, Copy)]
pub struct Nanoseconds(pub u32);

pub trait TimerExt {
    fn timers(self, frequency: u32) -> (Timer1, Timer2);
}

impl TimerExt for TIMER {
    fn timers(self, frequency: u32) -> (Timer1, Timer2) {
        (Timer1::new(frequency), Timer2::new(frequency))
    }
}

pub struct Timer1 {
    ticks_per_ms: u32,
}

impl Timer1 {
    fn new(frequency: u32) -> Self {
        let timer = unsafe { (&*TIMER::ptr()) };
        timer.frc1_ctrl.write(|w| {
            w.rollover()
                .set_bit()
                .interrupt_type()
                .level()
                .prescale_divider()
                .devided_by_256()
        });

        Timer1 {
            ticks_per_ms: (1_000_000_000 / (frequency / 256)),
        }
    }
}

impl CountDown for Timer1 {
    type Time = Nanoseconds;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Nanoseconds>,
    {
        let timer = unsafe { (&*TIMER::ptr()) };
        let timeout: Nanoseconds = timeout.into();

        let ticks = timeout.0 / self.ticks_per_ms;
        timer.frc1_ctrl.modify(|_, w| w.timer_enable().set_bit());
        timer.frc1_load.write(|w| unsafe { w.bits(ticks) });
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        let timer = unsafe { (&*TIMER::ptr()) };
        if timer.frc1_ctrl.read().frc1_int().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            timer
                .frc1_int
                .modify(|_, w| w.frc1_int_clr_mask().set_bit());
            Ok(())
        }
    }
}

impl Periodic for Timer1 {}

impl Cancel for Timer1 {
    type Error = Void;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        let timer = unsafe { (&*TIMER::ptr()) };
        timer.frc1_ctrl.modify(|_, w| w.timer_enable().clear_bit());
        Ok(())
    }
}

pub struct Timer2 {
    ticks_per_ms: u32,
}

impl Timer2 {
    fn new(frequency: u32) -> Self {
        let timer = unsafe { (&*TIMER::ptr()) };
        timer.frc2_ctrl.write(|w| {
            w.interrupt_type()
                .level()
                .prescale_divider()
                .devided_by_256()
        });

        Timer2 {
            ticks_per_ms: (1_000_000_000 / (frequency / 256)),
        }
    }
}

impl CountDown for Timer2 {
    type Time = Nanoseconds;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Nanoseconds>,
    {
        let timer = unsafe { (&*TIMER::ptr()) };
        let timeout: Nanoseconds = timeout.into();

        let ticks = timeout.0 / self.ticks_per_ms;
        timer.frc2_ctrl.modify(|_, w| w.timer_enable().set_bit());
        timer.frc2_load.write(|w| unsafe { w.bits(0) });
        timer.frc2_alarm.write(|w| unsafe { w.bits(ticks) });
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        let timer = unsafe { (&*TIMER::ptr()) };
        if timer.frc2_ctrl.read().frc2_int().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            timer
                .frc2_int
                .modify(|_, w| w.frc2_int_clr_mask().set_bit());
            Ok(())
        }
    }
}

impl Cancel for Timer2 {
    type Error = Void;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        let timer = unsafe { (&*TIMER::ptr()) };
        timer.frc2_ctrl.modify(|_, w| w.timer_enable().clear_bit());
        Ok(())
    }
}
