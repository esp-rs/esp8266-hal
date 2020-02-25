use embedded_hal::timer::CountDown;
use esp8266::timer::frc1_ctrl::PRESCALE_DIVIDER_A;
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
    start: u32,
    ticks: u32,
    ticks_per_ms: u32,
}

impl Timer {
    fn new(timer: TIMER, frequency: u32) -> Self {
        timer.frc1_ctrl.write(|w| {
            w.rollover()
                .set_bit()
                .timer_enable()
                .set_bit()
                .prescale_divider()
                .variant(PRESCALE_DIVIDER_A::DEVIDED_BY_256)
        });

        Timer {
            timer,
            start: 0,
            ticks: 0,
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

        // 3600 = 1e9 / (80MHz / 256)
        self.ticks = (timeout.0 / self.ticks_per_ms) as u32;
        self.start = self.timer.frc1_count.read().bits();
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        if (self.start - self.timer.frc1_count.read().bits() & 0x3fffff) < self.ticks {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}
