#![deny(unsafe_code)]
#![deny(warnings)]

use embedded_hal::blocking::delay::DelayMs;
use stm32f1xx_hal::time::Hertz;

/// Delay that is shitty and unpredictable, it could wait for much
/// more than requested. See also:
/// https://users.rust-lang.org/t/embedded-rtfm-timer-queue-blocking-wait/25369/5
///
/// Specific to STM32F1xx, unfortunately.
pub struct ShittyDelay {
    sysclk_freq: Hertz,
}

impl ShittyDelay {
    /// Construct ShittyDelay from `sysclk` frequency.
    pub fn new(sysclk_freq: Hertz) -> Self {
        ShittyDelay { sysclk_freq }
    }
}

impl DelayMs<u8> for ShittyDelay {
    fn delay_ms(&mut self, ms: u8) {
        cortex_m::asm::delay(self.sysclk_freq.0 / (1_000_000 * (ms as u32)));
    }
}
