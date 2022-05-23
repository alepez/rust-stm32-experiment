use stm32h7xx_hal::device::DWT;
use stm32h7xx_hal::time::Hertz;

pub struct SmartTimer {
    pub clock: Hertz,
}

impl SmartTimer {
    pub fn new(clock: Hertz) -> SmartTimer {
        SmartTimer { clock }
    }

    /// returns the time elapsed since the program started
    /// unit: us
    pub fn now_s(&self) -> f32 {
        let cycles = DWT::get_cycle_count();
        cycles as f32 / self.clock.0 as f32
    }

    pub fn now_ms(&self) -> f32 {
        let cycles = DWT::get_cycle_count();
        (cycles as f32 / self.clock.0 as f32) * 1_000 as f32
    }

    pub fn now_us(&self) -> f32 {
        let cycles = DWT::get_cycle_count();
        (cycles as f32 / self.clock.0 as f32) * 1_000_000 as f32
    }
}