use stm32h7xx_hal::device::DWT;
use stm32h7xx_hal::time::Hertz;

pub struct SmartTimer {
    clock: Hertz,
}

impl SmartTimer {
    pub fn new(clock: Hertz) -> SmartTimer {
        SmartTimer { clock }
    }

    /// returns the time elapsed since the program started
    /// unit: second
    pub fn now(&self) -> f32 {
        let cycles = DWT::get_cycle_count();
        cycles as f32 / self.clock.0 as f32
    }
}