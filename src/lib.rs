#![no_std]

use riscv_clic::peripheral::{SYST};
pub use fugit::{self, ExtU32};
use rtic_monotonic::Monotonic;

pub struct Systick<const TIMER_HZ: u32> {
    systick: SYST,
    cnt: u64,
}

impl<const TIMER_HZ: u32> Systick<TIMER_HZ> {
    /// Provide a new `Monotonic` based on SysTick.
    ///
    /// The `sysclk` parameter is the speed at which SysTick runs at. This value should come from
    /// the clock generation function of the used HAL.
    ///
    /// Notice that the actual rate of the timer is a best approximation based on the given
    /// `sysclk` and `TIMER_HZ`.
    pub fn new(mut systick: SYST, sysclk: u32) -> Self {

        systick.disable_cascaded_mode();
        
        systick.enable_interrupt_lo();

        systick.set_continuos_mode_lo();

        systick.disable_one_shot_mode_lo();

        systick.disable_lo();

        Systick { systick, cnt: 0 }
    }
}

impl<const TIMER_HZ: u32> Monotonic for Systick<TIMER_HZ> {
    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = false;

    type Instant = fugit::TimerInstantU32<TIMER_HZ>;
    type Duration = fugit::TimerDurationU32<TIMER_HZ>;

    fn now(&mut self) -> Self::Instant {
        Self::Instant::from_ticks(SYST::get_counter_lo())
    }

    fn set_compare(&mut self, instant: Self::Instant) {
       self.systick.set_compare_lo(instant.ticks());
    }

    fn clear_compare_flag(&mut self) {
        // not necessary as far as I know
    }

    fn zero() -> Self::Instant {
        Self::Instant::from_ticks(0)
    }

    unsafe fn reset(&mut self) {
        self.systick.set_counter_lo(0);
        self.systick.enable_lo();
    }

    fn on_interrupt(&mut self) {}

    fn enable_timer(&mut self) {
        self.systick.enable_lo();
    }

    fn disable_timer(&mut self) {
        self.systick.disable_lo();
    }
}