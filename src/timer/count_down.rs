use super::*;

use cast::u16;
use embedded_hal::timer::{Cancel, CountDown, Periodic};
use fugit::{MicrosDurationU32, TimerDurationU32, TimerInstantU32};
use void::Void;

/// Timer that waits given time
pub struct CountDownTimer<TIM, const FREQ: u32> {
    tim: TIM,
}

/// `CountDownTimer` with sampling of 1 MHz
pub type CountDownTimerUs<TIM> = CountDownTimer<TIM, 1_000_000>;

/// `CountDownTimer` with sampling of 1 kHz
///
/// NOTE: don't use this if your system frequency more than 65 MHz
pub type CountDownTimerMs<TIM> = CountDownTimer<TIM, 1_000>;

impl<TIM> Timer<TIM>
where
    TIM: Instance,
{
    /// Creates CountDownTimer with custom sampling
    pub fn count_down<const FREQ: u32>(self) -> CountDownTimer<TIM, FREQ> {
        let Self { tim, clk } = self;
        CountDownTimer::<TIM, FREQ>::new(tim, clk)
    }
    /// Creates CountDownTimer with sampling of 1 MHz
    pub fn count_down_us(self) -> CountDownTimerUs<TIM> {
        self.count_down::<1_000_000>()
    }

    /// Creates CountDownTimer with sampling of 1 kHz
    ///
    /// NOTE: don't use this if your system frequency more than 65 MHz
    pub fn count_down_ms(self) -> CountDownTimerMs<TIM> {
        self.count_down::<1_000>()
    }
}

impl<TIM, const FREQ: u32> Periodic for CountDownTimer<TIM, FREQ> {}

impl Timer<SYST> {
    /// Creates SysCountDownTimer
    pub fn count_down(self) -> SysCountDownTimer {
        let Self { tim, clk } = self;
        SysCountDownTimer::new(tim, clk)
    }
}

pub struct SysCountDownTimer {
    tim: SYST,
    mhz: u32,
}

impl SysCountDownTimer {
    fn new(tim: SYST, clk: Hertz) -> Self {
        Self {
            tim,
            mhz: clk.0 / 1_000_000,
        }
    }

    /// Starts listening for an `event`
    pub fn listen(&mut self, event: Event) {
        match event {
            Event::TimeOut => self.tim.enable_interrupt(),
        }
    }

    /// Stops listening for an `event`
    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::TimeOut => self.tim.disable_interrupt(),
        }
    }

    pub fn now(&self) -> TimerInstantU32<1_000_000> {
        TimerInstantU32::from_ticks((SYST::get_reload() - SYST::get_current()) / self.mhz)
    }

    pub fn start(&mut self, timeout: MicrosDurationU32) -> Result<(), Error> {
        let rvr = timeout.ticks() * self.mhz - 1;

        assert!(rvr < (1 << 24));

        self.tim.set_reload(rvr);
        self.tim.clear_current();
        self.tim.enable_counter();
        Ok(())
    }

    pub fn wait(&mut self) -> nb::Result<(), Error> {
        if self.tim.has_wrapped() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Error::Disabled);
        }

        self.tim.disable_counter();
        Ok(())
    }
}

impl CountDown for SysCountDownTimer {
    type Time = MicrosDurationU32;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Self::Time>,
    {
        self.start(timeout.into()).unwrap()
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        match self.wait() {
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            _ => Ok(()),
        }
    }
}

impl Cancel for SysCountDownTimer {
    type Error = Error;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.cancel()
    }
}

impl<TIM, const FREQ: u32> CountDownTimer<TIM, FREQ>
where
    TIM: General,
{
    fn new(mut tim: TIM, clk: Hertz) -> Self {
        let psc = clk.0 / FREQ - 1;
        tim.set_prescaler(u16(psc).unwrap());
        Self { tim }
    }

    /// Starts listening for an `event`
    ///
    /// Note, you will also have to enable the TIM2 interrupt in the NVIC to start
    /// receiving events.
    pub fn listen(&mut self, event: Event) {
        match event {
            Event::TimeOut => {
                // Enable update event interrupt
                self.tim.listen_update_interrupt(true);
            }
        }
    }

    /// Clears interrupt associated with `event`.
    ///
    /// If the interrupt is not cleared, it will immediately retrigger after
    /// the ISR has finished.
    pub fn clear_interrupt(&mut self, event: Event) {
        match event {
            Event::TimeOut => {
                // Clear interrupt flag
                self.tim.clear_update_interrupt_flag();
            }
        }
    }

    /// Stops listening for an `event`
    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::TimeOut => {
                // Disable update event interrupt
                self.tim.listen_update_interrupt(false);
            }
        }
    }

    /// Releases the TIM peripheral
    pub fn release(mut self) -> TIM {
        // pause counter
        self.tim.disable_counter();
        self.tim
    }

    pub fn now(&self) -> TimerInstantU32<FREQ> {
        TimerInstantU32::from_ticks(
            self.tim.read_auto_reload().into() - self.tim.read_count().into(),
        )
    }

    pub fn start(&mut self, timeout: TimerDurationU32<FREQ>) -> Result<(), Error> {
        // pause
        self.tim.disable_counter();
        // reset counter
        self.tim.reset_counter();

        let arr = timeout.ticks() - 1;
        self.tim.set_auto_reload(arr)?;

        // Trigger update event to load the registers
        self.tim.trigger_update();

        // start counter
        self.tim.enable_counter();

        Ok(())
    }

    pub fn wait(&mut self) -> nb::Result<(), Error> {
        if self.tim.get_update_interrupt_flag() {
            Err(nb::Error::WouldBlock)
        } else {
            self.tim.clear_update_interrupt_flag();
            Ok(())
        }
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if !self.tim.is_counter_enabled() {
            return Err(Error::Disabled);
        }

        // disable counter
        self.tim.disable_counter();
        Ok(())
    }
}

impl<TIM, const FREQ: u32> CountDown for CountDownTimer<TIM, FREQ>
where
    TIM: General,
{
    type Time = TimerDurationU32<FREQ>;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Self::Time>,
    {
        self.start(timeout.into()).unwrap()
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        match self.wait() {
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            _ => Ok(()),
        }
    }
}

impl<TIM, const FREQ: u32> Cancel for CountDownTimer<TIM, FREQ>
where
    TIM: General,
{
    type Error = Error;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.cancel()
    }
}

#[cfg(feature = "atat")]
impl atat::Clock<1_000_000> for SysCountDownTimer {
    type Error = Error;

    fn now(&mut self) -> TimerInstantU32<1_000_000> {
        Self::now(self)
    }

    fn start(&mut self, duration: MicrosDurationU32) -> Result<(), Self::Error> {
        self.start(duration)
    }

    fn wait(&mut self) -> nb::Result<(), Self::Error> {
        self.wait()
    }
}

#[cfg(feature = "atat")]
impl<TIM, const FREQ: u32> atat::Clock<FREQ> for CountDownTimer<TIM, FREQ>
where
    TIM: General,
{
    type Error = Error;

    fn now(&mut self) -> TimerInstantU32<FREQ> {
        Self::now(self)
    }

    fn start(&mut self, duration: TimerDurationU32<FREQ>) -> Result<(), Self::Error> {
        self.start(duration)
    }

    fn wait(&mut self) -> nb::Result<(), Self::Error> {
        self.wait()
    }
}
