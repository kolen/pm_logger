#![allow(deprecated)]

use embedded_hal::digital::v2::OutputPin;

/// Dummy embedded-hal output pin. Toggling it does nothing.
pub struct DummyOutputPin {
}

impl DummyOutputPin {
    pub fn new() -> Self { Self {  } }
}

impl OutputPin for DummyOutputPin {
    type Error = ();
    fn set_low(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn set_high(&mut self) -> Result<(), Self::Error> { Ok(()) }
}
