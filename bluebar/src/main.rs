#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

mod dummy_pin;

use dummy_pin::DummyOutputPin;

use bme280::BME280;
use cortex_m_semihosting::hprintln;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v1_compat::OldOutputPin;
use panic_semihosting as _;
use pcd8544::PCD8544;
use rtfm::cyccnt::{Duration, Instant, U32Ext};
use stm32f1xx_hal::{
    gpio,
    gpio::{gpioa, gpiob, Alternate, OpenDrain, PushPull},
    i2c, pac,
    prelude::*,
    time::Hertz,
};

/// Delay that is shitty and unpredictable, it could wait for much
/// more than requested. See also:
/// https://users.rust-lang.org/t/embedded-rtfm-timer-queue-blocking-wait/25369/5
pub struct ShittyDelay {
    sysclk_freq: Hertz,
}

impl ShittyDelay {
    pub fn new(sysclk_freq: Hertz) -> Self {
        ShittyDelay { sysclk_freq }
    }
}

impl DelayMs<u8> for ShittyDelay {
    fn delay_ms(&mut self, ms: u8) {
        cortex_m::asm::delay(self.sysclk_freq.0 / (1_000_000 * (ms as u32)));
    }
}

#[rtfm::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtfm::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        bme280: bme280::BME280<
            i2c::BlockingI2c<
                pac::I2C1,
                (
                    gpiob::PB8<Alternate<OpenDrain>>,
                    gpiob::PB9<Alternate<OpenDrain>>,
                ),
            >,
            ShittyDelay,
        >,
        display: pcd8544::PCD8544<
            OldOutputPin<gpiob::PB7<gpio::Output<PushPull>>>,
            OldOutputPin<gpiob::PB6<gpio::Output<PushPull>>>,
            OldOutputPin<gpiob::PB5<gpio::Output<PushPull>>>,
            OldOutputPin<DummyOutputPin>,
            OldOutputPin<gpioa::PA12<gpio::Output<PushPull>>>,
            OldOutputPin<DummyOutputPin>,
        >,
        period: Duration,
    }

    #[init(schedule = [periodic_measure])]
    fn init(mut cx: init::Context) -> init::LateResources {
        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        // This thing probably configures alternate mode of
        // pins. Hmm...
        let mut afio = cx.device.AFIO.constrain(&mut rcc.apb2);
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.apb2);
        let mut gpiob = cx.device.GPIOB.split(&mut rcc.apb2);
        let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
        let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

        let i2c = i2c::I2c::i2c1(
            cx.device.I2C1,
            (scl, sda),
            &mut afio.mapr,
            i2c::Mode::standard(1.khz()),
            clocks,
            &mut rcc.apb1,
        );
        let blocking_i2c = i2c::blocking_i2c(i2c, clocks, 1_000_000, 10, 1_000_000, 1_000_000);
        let delay = ShittyDelay::new(clocks.sysclk());
        let mut bme280 = BME280::new_primary(blocking_i2c, delay);
        bme280.init().expect("Init failed");

        // TODO: find a better timer, this ticks at unknown rate and
        // sucks
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        // TODO: is there cleaner way?
        let period = (clocks.sysclk().0 * 10).cycles();

        hprintln!("init").ok();
        hprintln!("schedule period: {}", period.as_cycles()).ok();
        hprintln!("init @ {:?}", cx.start).ok();

        cx.schedule.periodic_measure(cx.start + period).unwrap();

        hprintln!("Schedule ok").ok();

        let pin_clk: OldOutputPin<_> = gpiob.pb7.into_push_pull_output(&mut gpiob.crl).into();
        let pin_din: OldOutputPin<_> = gpiob.pb6.into_push_pull_output(&mut gpiob.crl).into();
        let pin_dc: OldOutputPin<_> = gpiob.pb5.into_push_pull_output(&mut gpiob.crl).into();
        let pin_rst: OldOutputPin<_> = gpioa.pa12.into_push_pull_output(&mut gpioa.crh).into();

        // CE: pulled up to Vcc
        let dummy_ce: OldOutputPin<_> = DummyOutputPin::new().into();
        // Light: floating for now
        let dummy_light: OldOutputPin<_> = DummyOutputPin::new().into();

        let display = PCD8544::new(pin_clk, pin_din, pin_dc, dummy_ce, pin_rst, dummy_light);

        init::LateResources {
            bme280,
            display,
            period,
        }
    }

    #[task(schedule = [periodic_measure], resources=[period, bme280])]
    fn periodic_measure(cx: periodic_measure::Context) {
        hprintln!("periodic_measure").ok();
        let now = Instant::now();
        hprintln!("scheduled = {:?}, now = {:?}", cx.scheduled, now).unwrap();

        let bme280 = cx.resources.bme280;
        let measurements = bme280.measure().expect("Measure failed");

        hprintln!("Relative Humidity = {}%", measurements.humidity).ok();
        hprintln!("Temperature = {} deg C", measurements.temperature).ok();
        hprintln!("Pressure = {} pascals", measurements.pressure).ok();

        cx.schedule
            .periodic_measure(cx.scheduled + *cx.resources.period)
            .unwrap();
    }

    extern "C" {
        fn DMA1_CHANNEL1();
    }
};
