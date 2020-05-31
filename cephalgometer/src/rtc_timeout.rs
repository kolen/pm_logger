use embedded_hal::timer;
use stm32f1xx_hal::rtc::Rtc as STM32Rtc;
use void::Void;

pub struct RTCTimeout {
    rtc: *const STM32Rtc,
    pend_time: u32,
}

impl RTCTimeout {
    /// Construct new RTCTimeout. RTC pointer must be always valid!
    pub fn new(rtc: *const STM32Rtc) -> Self {
        RTCTimeout { rtc, pend_time: 0 }
    }
}

impl timer::CountDown for RTCTimeout {
    type Time = u32;

    fn start<T>(&mut self, count: T)
    where
        T: Into<Self::Time>,
    {
        unsafe {
            self.pend_time = (&*self.rtc).current_time() + count.into();
        }
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        if unsafe { (&*self.rtc).current_time() >= self.pend_time } {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

// hacky AF
unsafe impl Send for RTCTimeout {}
