#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod dummy_output_pin;
mod shitty_delay;

use bme280::BME280;
use cortex_m_semihosting::hprintln;
use mh_z_rr::MH_Z_RR;
use panic_semihosting as _;
use pcd8544::PCD8544;
use rtfm::cyccnt::{Duration, Instant, U32Ext};
use shitty_delay::ShittyDelay;
use stm32f1xx_hal::gpio::gpioa::{PA11, PA12};
use stm32f1xx_hal::gpio::gpiob::{PB5, PB6, PB7, PB8, PB9};
use stm32f1xx_hal::gpio::{Alternate, OpenDrain, Output, PushPull};
use stm32f1xx_hal::serial::{self, Serial};
use stm32f1xx_hal::stm32::{USART1, TIM2};
use stm32f1xx_hal::{i2c, pac, prelude::*};
use stm32f1xx_hal::timer::{Timer, CountDownTimer};
use nb::block;

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
        timeout: CountDownTimer<TIM2>,
        mh_z: MH_Z_RR<serial::Rx<USART1>, serial::Tx<USART1>>,
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

        // -------------- TODO: extract --------------------

        let pcd_clk = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);
        let pcd_din = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);
        let pcd_dc = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);
        let pcd_ce = gpioa.pa11.into_push_pull_output(&mut gpioa.crh);
        let pcd_rst = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
        let pcd_light = dummy_output_pin::DummyOutputPin::new();

        // clk din dc ce rst light
        let pcd8544 = PCD8544::new(pcd_clk, pcd_din, pcd_dc, pcd_ce, pcd_rst, pcd_light).unwrap();
        // pins can't error on stm32, hopefully unwrap formatting code
        // will be removed by compiler

        // ---------------- TODO: extract -----------------------------

        let mh_tx_pin = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
        let mh_rx_pin = gpioa.pa10;
        let mh_serial = Serial::usart1(
            cx.device.USART1,
            (mh_tx_pin, mh_rx_pin),
            &mut afio.mapr,
            serial::Config::default().baudrate(9_600.bps()),
            clocks,
            &mut rcc.apb2,
        );
        let (mh_tx, mh_rx) = mh_serial.split();
        let mh_z = MH_Z_RR::new(mh_rx, mh_tx);

        let timeout = Timer::tim2(cx.device.TIM2, &clocks, &mut rcc.apb1).start_count_down(1.hz());

        init::LateResources {
            bme280,
            period,
            pcd8544,
            timeout,
            mh_z,
        }
    }

    #[task(schedule = [periodic_measure], resources=[period, bme280, mh_z, timeout])]
    fn periodic_measure(cx: periodic_measure::Context) {
        hprintln!("periodic_measure").ok();
        let now = Instant::now();
        hprintln!("scheduled = {:?}, now = {:?}", cx.scheduled, now).unwrap();

        let bme280 = cx.resources.bme280;
        let measurements = bme280.measure().expect("Measure failed");
        hprintln!("Relative Humidity = {}%", measurements.humidity).ok();
        hprintln!("Temperature = {} deg C", measurements.temperature).ok();
        hprintln!("Pressure = {} pascals", measurements.pressure).ok();

        cx.resources.timeout.reset();
        hprintln!("Timer reset").ok();
        let mut co2_runner = cx.resources.mh_z.read_gas_concentration(1, cx.resources.timeout);
        hprintln!("read_gas_concentration ok").ok();
        match block!(co2_runner.run()) {
            Ok(co2) => hprintln!("CO2 = {} PPM", co2).ok(),
            Err(e) => hprintln!("CO2 measure failed, {:?}", e).ok(),
        };

        cx.schedule
            .periodic_measure(cx.scheduled + *cx.resources.period)
            .unwrap();
    }

    extern "C" {
        fn DMA1_CHANNEL1();
    }
};
