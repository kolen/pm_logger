#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use bme280::BME280;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use stm32f1xx_hal::{delay::Delay, gpio, i2c, pac, prelude::*, time::KiloHertz};

#[rtfm::app(device = stm32f1xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        bme280: bme280::BME280<
            i2c::BlockingI2c<
                pac::I2C1,
                (
                    gpio::gpiob::PB8<gpio::Alternate<gpio::OpenDrain>>,
                    gpio::gpiob::PB9<gpio::Alternate<gpio::OpenDrain>>,
                ),
            >,
            stm32f1xx_hal::delay::Delay,
        >,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();
        // Dunno what I do and how this works
        let clocks = rcc.cfgr.adcclk(2.mhz()).freeze(&mut flash.acr);

        // This thing probably configures alternate mode of
        // pins. Hmm...
        let mut afio = cx.device.AFIO.constrain(&mut rcc.apb2);
        let mut gpiob = cx.device.GPIOB.split(&mut rcc.apb2);
        let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
        let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

        let i2c = i2c::I2c::i2c1(
            cx.device.I2C1,
            (scl, sda),
            &mut afio.mapr,
            i2c::Mode::standard(KiloHertz(1)),
            clocks,
            &mut rcc.apb1,
        );
        let blocking_i2c = i2c::blocking_i2c(i2c, clocks, 1_000, 10, 1_000, 1_000);
        let delay = Delay::new(cx.core.SYST, clocks);
        let bme280 = BME280::new_primary(blocking_i2c, delay);

        hprintln!("init").unwrap();

        init::LateResources { bme280 }
    }

    #[idle(resources = [bme280])]
    fn idle(cx: idle::Context) -> ! {
        let bme280 = cx.resources.bme280;
        hprintln!("Initializing").ok();
        // Looks like we need some time before init. Adding this
        // hprintln! makes it working, without it it fails with
        // I2c(Acknowledge). Or to tweak some timeouts.
        bme280.init().unwrap();
        hprintln!("Measuring").ok();
        let measurements = bme280.measure().unwrap();

        hprintln!("Relative Humidity = {}%", measurements.humidity).ok();
        hprintln!("Temperature = {} deg C", measurements.temperature).ok();
        hprintln!("Pressure = {} pascals", measurements.pressure).ok();

        loop {}
    }
};
