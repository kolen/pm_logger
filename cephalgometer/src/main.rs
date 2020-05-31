#![no_main]
#![no_std]

mod dummy_output_pin;
mod rtc_timeout;
mod shitty_delay;

use bme280::BME280;
use core::fmt::Write;
use core::mem;
use cortex_m_semihosting::hprintln;
use mh_z_rr::MH_Z_RR;
use nb::block;
use panic_semihosting as _;
use pcd8544::PCD8544;
use rtc_timeout::RTCTimeout;
use rtfm::cyccnt::{Duration, Instant, U32Ext};
use shitty_delay::ShittyDelay;
use stm32f1xx_hal::gpio::gpioa::{PA11, PA12};
use stm32f1xx_hal::gpio::gpiob::{PB5, PB6, PB7, PB8, PB9};
use stm32f1xx_hal::gpio::{Alternate, OpenDrain, Output, PushPull};
use stm32f1xx_hal::rtc::Rtc;
use stm32f1xx_hal::serial::{self, Serial};
use stm32f1xx_hal::stm32::USART2;
use stm32f1xx_hal::{i2c, pac, prelude::*};

#[rtfm::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtfm::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        bme280: bme280::BME280<
            i2c::BlockingI2c<pac::I2C1, (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>)>,
            ShittyDelay,
        >,
        period: Duration,
        // PCD8544<CLK, DIN, DC, CE, RST, LIGHT>
        pcd8544: PCD8544<
            PB7<Output<PushPull>>,
            PB6<Output<PushPull>>,
            PB5<Output<PushPull>>,
            PA11<Output<PushPull>>,
            PA12<Output<PushPull>>,
            dummy_output_pin::DummyOutputPin,
        >,
        timeout: RTCTimeout,
        mh_z: MH_Z_RR<serial::Rx<USART2>, serial::Tx<USART2>>,
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
        // sucks. Use RTC probably.
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        // TODO: is there cleaner way?
        let period = (clocks.sysclk().0 * 10).cycles();

        // TODO: use RTC
        hprintln!("schedule period: {}", period.as_cycles()).ok();
        cx.schedule.periodic_measure(cx.start + period).unwrap();
        hprintln!("Schedule ok").ok();

        // -------------- TODO: extract --------------------

        let pcd_clk = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);
        let pcd_din = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);
        let pcd_dc = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);
        let pcd_ce = gpioa.pa11.into_push_pull_output(&mut gpioa.crh);
        let pcd_rst = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
        let pcd_light = dummy_output_pin::DummyOutputPin::new();

        // clk din dc ce rst light
        let mut pcd8544 =
            PCD8544::new(pcd_clk, pcd_din, pcd_dc, pcd_ce, pcd_rst, pcd_light).unwrap();
        // pins can't error on stm32, hopefully unwrap formatting code
        // will be removed by compiler

        pcd8544.clear().unwrap();

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

        let mh_z = MH_Z_RR::new(mh_rx, mh_tx);

        // Set up RTC
        let mut backup_domain = rcc
            .bkp
            .constrain(cx.device.BKP, &mut rcc.apb1, &mut cx.device.PWR);
        let rtc = Rtc::rtc(cx.device.RTC, &mut backup_domain);
        let timeout = RTCTimeout::new(&rtc);
        mem::forget(rtc);

        init::LateResources {
            bme280,
            period,
            pcd8544,
            timeout,
            mh_z,
        }
    }

    #[task(schedule = [periodic_measure], resources=[period, bme280, pcd8544, mh_z, timeout])]
    fn periodic_measure(cx: periodic_measure::Context) {
        hprintln!("periodic_measure").ok();
        let now = Instant::now();
        hprintln!("scheduled = {:?}, now = {:?}", cx.scheduled, now).unwrap();

        let bme280 = cx.resources.bme280;
        let measurements = bme280.measure().expect("Measure failed");
        hprintln!("Relative Humidity = {}%", measurements.humidity).ok();
        hprintln!("Temperature = {} deg C", measurements.temperature).ok();
        hprintln!("Pressure = {} pascals", measurements.pressure).ok();

        cx.resources.timeout.start(5u32); // FIXME: baad API, probably shouldn't use timer trait
        let mut co2_runner = cx
            .resources
            .mh_z
            .read_gas_concentration(1, cx.resources.timeout);
        let mut concentration: Option<u32> = None;
        match block!(co2_runner.run()) {
            Ok(co2) => {
                hprintln!("CO2 = {} PPM", co2).ok();
                concentration = Some(co2);
            }
            Err(e) => {
                hprintln!("CO2 measure failed, {:?}", e).ok();
            }
        };

        let pcd = cx.resources.pcd8544;
        pcd.reset().unwrap();
        writeln!(
            pcd,
            "{} C, {} %",
            measurements.temperature, measurements.humidity
        )
        .unwrap();
        writeln!(pcd, "{} pas", measurements.pressure).unwrap();
        match concentration {
            Some(conc) => {
                writeln!(pcd, "{} PPM", conc).unwrap();
            }
            None => {
                writeln!(pcd, "Can't read CO2").unwrap();
            }
        }

        cx.schedule
            .periodic_measure(cx.scheduled + *cx.resources.period)
            .unwrap();
    }

    extern "C" {
        fn DMA1_CHANNEL1();
    }
};
