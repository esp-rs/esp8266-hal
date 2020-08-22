use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::timer::{Cancel, CountDown, Periodic};
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

macro_rules! impl_timer {
    ($TIMER:ident: ($ctrl:ident, $load:ident, $alarm:ident, $int:ident, $clr_mask:ident, $load_value:ident)) => {
        pub struct $TIMER {
            ticks_per_ms: u32,
        }

        impl $TIMER {
            fn new(frequency: u32) -> Self {
                let timer = unsafe { (&*TIMER::ptr()) };
                timer.$ctrl.write(|w| {
                    w.rollover()
                        .set_bit()
                        .interrupt_type()
                        .level()
                        .prescale_divider()
                        .devided_by_256()
                });
                timer.$alarm.write(|w| unsafe { w.bits(0) });

                $TIMER {
                    ticks_per_ms: (1_000_000_000 / (frequency / 256)),
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

        impl DelayUs<u32> for $TIMER {
            fn delay_us(&mut self, ms: u32) {
                self.start(Nanoseconds(ms));
                nb::block!(self.wait()).unwrap()
            }
        }

        impl DelayMs<u32> for $TIMER {
            fn delay_ms(&mut self, ms: u32) {
                self.delay_us(ms * 1000_000);
            }
        }
    };
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
