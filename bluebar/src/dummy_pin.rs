use embedded_hal::digital::v2::OutputPin;

/// Dummy output pin that does nothing. Useful for libs that require
/// to supply pin for e.g. 'chip select', and you already pulled up
/// chip select to Vcc and don't need a pin for that. Just pass a
/// DummyPin to such lib and it will do nothing.
pub struct DummyOutputPin {}

impl DummyOutputPin {
    pub fn new() -> Self {
        DummyOutputPin {}
    }
}

impl OutputPin for DummyOutputPin {
    type Error = ();
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
