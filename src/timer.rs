use crate::time::{Hertz, MegaHertz, Microseconds, Nanoseconds};

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::timer::{Cancel, CountDown, Periodic};
use esp8266::{DPORT, TIMER};
use void::Void;

pub trait TimerExt: Sized {
    fn timers(self) -> (Timer1, Timer2) {
        self.with_clock_frequency(MegaHertz(80))
    }

    /// Configure the timer with a non default clock frequency
    fn with_clock_frequency<T>(self, frequency: T) -> (Timer1, Timer2)
    where
        T: Into<Hertz>;
}

impl TimerExt for TIMER {
    fn with_clock_frequency<T>(self, frequency: T) -> (Timer1, Timer2)
    where
        T: Into<Hertz>,
    {
        let frequency: Hertz = frequency.into();
        (Timer1::new(frequency), Timer2::new(frequency))
    }
}

macro_rules! impl_timer {
    ($TIMER:ident: ($ctrl:ident, $load:ident, $alarm:ident, $int:ident, $clr_mask:ident, $load_value:ident)) => {
        pub struct $TIMER {
            ticks_per_ms: u32,
        }

        impl $TIMER {
            fn new<T>(frequency: T) -> Self
            where
                T: Into<Hertz>,
            {
                let timer = unsafe { (&*TIMER::ptr()) };
                timer.$ctrl.write(|w| {
                    w.rollover()
                        .set_bit()
                        .interrupt_type()
                        .level()
                        .prescale_divider()
                        .devided_by_256()
                        .timer_enable()
                        .clear_bit()
                });
                timer.$alarm.write(|w| unsafe { w.bits(0) });
                timer.$int.modify(|_, w| w.$clr_mask().set_bit());

                let frequency: Hertz = frequency.into();
                $TIMER {
                    ticks_per_ms: (1_000_000_000 / (frequency.0 / 256)),
                }
            }
        }

        impl CountDown for $TIMER {
            type Time = Nanoseconds;

            fn start<T>(&mut self, timeout: T)
            where
                T: Into<Nanoseconds>,
            {
                let timer = unsafe { (&*TIMER::ptr()) };
                let timeout: Nanoseconds = timeout.into();

                let ticks = timeout.0 / self.ticks_per_ms;
                timer.$ctrl.modify(|_, w| w.timer_enable().set_bit());
                timer.$load.write(|w| unsafe { w.bits($load_value(ticks)) });
                timer.$int.modify(|_, w| w.$clr_mask().set_bit());
            }

            fn wait(&mut self) -> nb::Result<(), Void> {
                let timer = unsafe { (&*TIMER::ptr()) };
                if timer.$ctrl.read().$int().bit_is_clear() {
                    Err(nb::Error::WouldBlock)
                } else {
                    timer.$int.modify(|_, w| w.$clr_mask().set_bit());
                    Ok(())
                }
            }
        }

        impl Periodic for $TIMER {}

        impl Cancel for $TIMER {
            type Error = Void;

            fn cancel(&mut self) -> Result<(), Self::Error> {
                let timer = unsafe { (&*TIMER::ptr()) };
                timer.$ctrl.modify(|_, w| w.timer_enable().clear_bit());
                Ok(())
            }
        }

        impl_timer_delay!($TIMER, i32);
        impl_timer_delay!($TIMER, u32);
        impl_timer_delay!($TIMER, u16);
        impl_timer_delay!($TIMER, u8);
    };
}

macro_rules! impl_timer_delay {
    ($TIMER:ident, $ty:ty) => {
        impl DelayUs<$ty> for $TIMER {
            fn delay_us(&mut self, us: $ty) {
                self.start(Microseconds(us as u32));
                nb::block!(self.wait()).unwrap()
            }
        }

        impl DelayMs<$ty> for $TIMER {
            fn delay_ms(&mut self, ms: $ty) {
                self.delay_us(ms as u32 * 1_000);
            }
        }
    };
}

impl Timer1 {
    /// Enable edge interrupts for this timer
    ///
    /// Note that using `wait` on the timer in unreliable why interrupts are enabled
    pub fn enable_interrupts(&self) {
        let timer = unsafe { &*TIMER::ptr() };
        let dport = unsafe { &*DPORT::ptr() };

        timer.frc1_ctrl.modify(|_, w| w.interrupt_type().edge());
        dport
            .edge_int_enable
            .modify(|_, w| w.timer1_edge_int_enable().set_bit());
    }

    pub fn disable_interrupts(&self) {
        let timer = unsafe { &*TIMER::ptr() };
        let dport = unsafe { &*DPORT::ptr() };

        timer.frc1_ctrl.modify(|_, w| w.interrupt_type().level());
        dport
            .edge_int_enable
            .modify(|_, w| w.timer1_edge_int_enable().clear_bit());
    }
}

fn timer1_load_value(ticks: u32) -> u32 {
    ticks
}

fn timer2_load_value(ticks: u32) -> u32 {
    // timer2 counts up
    u32::max_value() - ticks
}

impl_timer!(
    Timer1:
        (
            frc1_ctrl,
            frc1_load,
            frc1_load,
            frc1_int,
            frc1_int_clr_mask,
            timer1_load_value
        )
);

impl_timer!(
    Timer2:
        (
            frc2_ctrl,
            frc2_load,
            frc2_alarm,
            frc2_int,
            frc2_int_clr_mask,
            timer2_load_value
        )
);
