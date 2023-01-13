#![no_std]

use riscv_clic::peripheral::{SYST};
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

        systick.enable_cascaded_mode();
        
        //systick.set_compare_lo(0x8000);

        systick.enable_interrupt_lo();
        //systick.enable_interrupt_hi();

        systick.set_continuos_mode_lo();
        systick.set_continuos_mode_hi();

        systick.disable_one_shot_mode_lo();
        systick.disable_one_shot_mode_hi();

        systick.disable_lo();
        systick.disable_hi();

        Systick { systick, cnt: 0 }
    }
}

impl<const TIMER_HZ: u32> Monotonic for Systick<TIMER_HZ> {
    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = true;

    type Instant = fugit::TimerInstantU64<TIMER_HZ>;
    type Duration = fugit::TimerDurationU64<TIMER_HZ>;

    fn now(&mut self) -> Self::Instant {
        let mut val_hi = SYST::get_counter_hi();
        let mut val_lo = SYST::get_counter_lo();

        // load lo and hi until hi was twice the same value (no overflow)
        while SYST::get_counter_hi() != val_hi {
            val_hi = SYST::get_counter_hi();
            val_lo = SYST::get_counter_lo();
        }

        // put lo and hi bits together
        Self::Instant::from_ticks(((val_hi as u64) << 32) + (val_lo as u64))

    }

    fn set_compare(&mut self, instant: Self::Instant) {
        // TODO: Discuss best practice
        // first, set hi to a value not to reach so soon
        let val_hi = SYST::get_counter_hi()+2;
        self.systick.set_compare_hi(val_hi);

        // then, set the real lo and finally hi part
        self.systick.set_compare_lo((instant.ticks() & 0xFFFF_FFFF) as u32);
        self.systick.set_compare_hi((instant.ticks() >> 32) as u32);

    }

    fn clear_compare_flag(&mut self) {
        // not necessary as far as I know
    }

    fn zero() -> Self::Instant {
        Self::Instant::from_ticks(0)
    }

    unsafe fn reset(&mut self) {
        self.systick.set_counter_hi(0);
        self.systick.set_counter_lo(0);
    }

    fn on_interrupt(&mut self) {}

    fn enable_timer(&mut self) {
        self.systick.enable_hi();
        self.systick.enable_lo();
    }

    fn disable_timer(&mut self) {
        self.systick.disable_lo();
        self.systick.disable_hi();
    }
}