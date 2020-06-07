use arrayvec::ArrayString;
use core::fmt::Write;
use embedded_graphics::fonts::Font8x16;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::{egtext, text_style};
use embedded_hal::blocking;
use embedded_hal::digital::v2::OutputPin;
use pcd8544::PCD8544;
use pcd8544_embedded_graphics::PCD8544EmbeddedGraphics;

pub struct Measurements {
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
    pub co2: Option<u32>,
}

pub fn display<SPI, DC, CE, RST, LIGHT>(
    pcd8544: &mut PCD8544<SPI, DC, CE, RST, LIGHT>,
    measurements: &Measurements,
) where
    SPI: blocking::spi::Write<u8>,
    DC: OutputPin,
    CE: OutputPin,
    RST: OutputPin,
    LIGHT: OutputPin,
{
    let mut display = PCD8544EmbeddedGraphics::new();

    let mut buf = ArrayString::<[_; 24]>::new();
    // We have 10 chars width
    write!(
        &mut buf,
        "{:4.1}°C {:2.0}%\n{:6.0} Pa\n{:5} PPM",
        measurements.temperature,
        measurements.humidity,
        measurements.pressure,
        measurements.co2.unwrap_or(0)
    )
    .unwrap();

    egtext!(
        text = &buf,
        top_left = Point::zero(),
        style = text_style!(
            font = Font8x16,
            text_color = BinaryColor::On,
            background_color = BinaryColor::Off,
        )
    )
    .draw(&mut display)
    .unwrap();
    display.flush(pcd8544).map_err(|_| "Can't flush").unwrap();
}
