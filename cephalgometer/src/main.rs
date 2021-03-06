#![no_main]
#![no_std]

mod display;
mod dummy_output_pin;
mod measurements_store;
mod rtc_timeout;
mod shitty_delay;

use bme280::BME280;
use core::mem;
use embedded_hal::digital::v2::InputPin;
use mh_z_rr::MH_Z_RR;
use nb::block;
use panic_semihosting as _;
use pcd8544::PCD8544;
use rtc_timeout::RTCTimeout;
use rtic::cyccnt::{Duration, U32Ext};
use shitty_delay::ShittyDelay;
use stm32f1xx_hal::gpio::gpioa::{PA7, PA8};
use stm32f1xx_hal::gpio::gpiob::{PB12, PB13, PB14, PB15, PB8, PB9};
use stm32f1xx_hal::gpio::{Alternate, Floating, Input, OpenDrain, Output, PushPull};
use stm32f1xx_hal::rtc::Rtc;
use stm32f1xx_hal::serial::{self, Serial};
use stm32f1xx_hal::spi;
use stm32f1xx_hal::stm32::{SPI2, USART2};
use stm32f1xx_hal::{i2c, pac, prelude::*};

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        bme280: bme280::BME280<
            i2c::BlockingI2c<pac::I2C1, (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>)>,
            ShittyDelay,
        >,
        period: Duration,
        // PCD8544<CLK, DIN, DC, CE, RST, LIGHT>
        pcd8544: PCD8544<
            spi::Spi<
                SPI2,
                spi::Spi2NoRemap,
                (
                    spi::NoMiso,
                    PB15<Alternate<PushPull>>,
                    PB13<Alternate<PushPull>>,
                ),
            >,
            PB14<Output<PushPull>>,
            PB12<Output<PushPull>>,
            PA8<Output<PushPull>>,
            dummy_output_pin::DummyOutputPin,
        >,
        timeout: RTCTimeout,
        mh_z: MH_Z_RR<serial::Rx<USART2>, serial::Tx<USART2>>,
        pin_calibrate_zero: PA7<Input<Floating>>,
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

        let i2c = i2c::BlockingI2c::i2c1(
            cx.device.I2C1,
            (scl, sda),
            &mut afio.mapr,
            i2c::Mode::standard(1.khz()),
            clocks,
            &mut rcc.apb1,
            1_000,
            5,
            1_000,
            1_000,
        );

        let delay = ShittyDelay::new(clocks.sysclk());
        let mut bme280 = BME280::new_primary(i2c, delay);
        bme280.init().expect("Init failed");

        // TODO: find a better timer, this ticks at unknown rate and
        // sucks. Use RTC probably.
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        // TODO: is there cleaner way?
        let period = (clocks.sysclk().0 * 10).cycles();

        // TODO: use RTC
        cx.schedule.periodic_measure(cx.start + period).unwrap();

        // -------------- TODO: extract --------------------

        let pcd_sck = gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh);
        let pcd_mosi = gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh);

        let pcd_dc = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
        let pcd_ce = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
        let pcd_rst = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);
        let pcd_light = dummy_output_pin::DummyOutputPin::new();

        let spi_mode = spi::Mode {
            polarity: spi::Polarity::IdleLow,
            phase: spi::Phase::CaptureOnFirstTransition,
        };
        let spi_pins = (spi::NoMiso {}, pcd_mosi, pcd_sck);
        let spi = spi::Spi::spi2(
            cx.device.SPI2,
            spi_pins,
            spi_mode,
            100.khz(),
            clocks,
            &mut rcc.apb1,
        );

        // clk din dc ce rst light
        let mut pcd8544 = PCD8544::new(spi, pcd_dc, pcd_ce, pcd_rst, pcd_light).unwrap();
        // pins can't error on stm32, hopefully unwrap formatting code
        // will be removed by compiler

        pcd8544.reset().unwrap();

        // ---------------- TODO: extract -----------------------------

        let mh_tx_pin = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
        let mh_rx_pin = gpioa.pa3;
        let mh_serial = Serial::usart2(
            cx.device.USART2,
            (mh_tx_pin, mh_rx_pin),
            &mut afio.mapr,
            serial::Config::default().baudrate(9_600.bps()),
            clocks,
            &mut rcc.apb1,
        );
        let (mh_tx, mut mh_rx) = mh_serial.split();
        // TODO: check necessity, the idea was that it resets overrun
        // flag before actual read is performed
        mh_rx.read().ok();

        let mut mh_z = MH_Z_RR::new(mh_rx, mh_tx);

        // Set up RTC
        let mut backup_domain = rcc
            .bkp
            .constrain(cx.device.BKP, &mut rcc.apb1, &mut cx.device.PWR);
        let rtc = Rtc::rtc(cx.device.RTC, &mut backup_domain);
        let mut timeout = RTCTimeout::new(&rtc);
        mem::forget(rtc);

        timeout.start(5u32);
        mh_z.set_automatic_baseline_correction(1, false, &mut timeout)
            .unwrap();

        let pin_calibrate_zero = gpioa.pa7;

        init::LateResources {
            bme280,
            period,
            pcd8544,
            timeout,
            mh_z,
            pin_calibrate_zero,
        }
    }

    #[task(schedule = [periodic_measure], resources=[period, bme280, pcd8544, mh_z, timeout, pin_calibrate_zero])]
    fn periodic_measure(cx: periodic_measure::Context) {
        // hprintln!("periodic_measure").ok();
        // let now = Instant::now();
        // hprintln!("scheduled = {:?}, now = {:?}", cx.scheduled, now).unwrap();

        let bme280 = cx.resources.bme280;
        let measurements = bme280.measure().expect("Measure failed");
        // hprintln!("Relative Humidity = {}%", measurements.humidity).ok();
        // hprintln!("Temperature = {} deg C", measurements.temperature).ok();
        // hprintln!("Pressure = {} pascals", measurements.pressure).ok();

        if cx.resources.pin_calibrate_zero.is_high().unwrap() {
            cx.resources.timeout.start(5u32);
            cx.resources
                .mh_z
                .calibrate_zero_point(1, cx.resources.timeout)
                .unwrap();
        }

        cx.resources.timeout.start(5u32); // FIXME: baad API, probably shouldn't use timer trait
        let mut co2_runner = cx
            .resources
            .mh_z
            .read_gas_concentration(1, cx.resources.timeout);
        let mut co2: Option<u32> = None;
        match block!(co2_runner.run()) {
            Ok(co2_) => {
                // hprintln!("CO2 = {} PPM", co2_).ok();
                co2 = Some(co2_);
            }
            Err(_e) => {
                // hprintln!("CO2 measure failed, {:?}", e).ok();
            }
        };

        let pcd = cx.resources.pcd8544;

        display::display(
            pcd,
            &display::Measurements {
                temperature: measurements.temperature,
                humidity: measurements.humidity,
                pressure: measurements.pressure,
                co2,
            },
        );

        cx.schedule
            .periodic_measure(cx.scheduled + *cx.resources.period)
            .unwrap();
    }

    extern "C" {
        fn DMA1_CHANNEL1();
    }
};
