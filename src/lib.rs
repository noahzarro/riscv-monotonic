#![no_std]

use riscv_clic::peripheral::SYST;
pub use fugit::{self, ExtU64};
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
        // + TIMER_HZ / 2 provides round to nearest instead of round to 0.
        // - 1 as the counter range is inclusive [0, reload]
        let reload = (sysclk + TIMER_HZ / 2) / TIMER_HZ - 1;

        assert!(reload <= 0x00ff_ffff);
        assert!(reload > 0);

        systick.disable_cascaded_mode();
        
        systick.set_compare_lo(0x8000);

        systick.enable_interrupt_lo();

        systick.set_cycle_mode_lo();

        systick.disable_lo();

        Systick { systick, cnt: 0 }
    }
}

impl<const TIMER_HZ: u32> Monotonic for Systick<TIMER_HZ> {
    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = true;

    type Instant = fugit::TimerInstantU64<TIMER_HZ>;
    type Duration = fugit::TimerDurationU64<TIMER_HZ>;

    fn now(&mut self) -> Self::Instant {
        todo!()
    }

    fn set_compare(&mut self, instant: Self::Instant) {
        todo!()
    }

    fn clear_compare_flag(&mut self) {
        todo!()
    }

    fn zero() -> Self::Instant {
        todo!()
    }

    unsafe fn reset(&mut self) {
        todo!()
    }

    fn on_interrupt(&mut self) {}

    fn enable_timer(&mut self) {}

    fn disable_timer(&mut self) {}
}