#![no_std]

//! Adds
//! [`embedded-graphics`](https://crates.io/crates/embedded-graphics)
//! interface support for PCD8544.

use core::convert::TryInto;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_hal::blocking;
use embedded_hal::digital::v2::OutputPin;
use pcd8544::{self, PCD8544};

const WIDTH: u32 = 84;
const HEIGHT: u32 = 48;
const MAX_X: u32 = WIDTH - 1;
const MAX_Y: u32 = HEIGHT - 1;
const HEIGHT_BANKS: usize = 6;

/// Implements `embedded-graphics` interface for PCD8544.
///
/// It does not draw on PCD8544 directly. This struct holds frame
/// buffer allowing to draw on it with `embedded-graphics` and libs
/// compatible with it. After drawing, call `flush` to transfer
/// framebuffer data to PCD8544 display at once.
pub struct PCD8544EmbeddedGraphics {
    framebuffer: [[u8; WIDTH as usize]; HEIGHT_BANKS],
}

impl PCD8544EmbeddedGraphics {
    /// Creates new PCD8544EmbeddedGraphics with frame buffer filled
    /// with white color (unset pixels).
    pub fn new() -> Self {
        PCD8544EmbeddedGraphics {
            framebuffer: [[0; WIDTH as usize]; HEIGHT_BANKS],
        }
    }

    /// Transfers internal framebuffer data to PCD8544.
    pub fn flush<SPI, DC, CE, RST, LIGHT>(
        &self,
        pcd8544: &mut PCD8544<SPI, DC, CE, RST, LIGHT>,
    ) -> Result<(), pcd8544::OutputError<SPI::Error, DC::Error, CE::Error, RST::Error, LIGHT::Error>>
    where
        SPI: blocking::spi::Write<u8>,
        DC: OutputPin,
        CE: OutputPin,
        RST: OutputPin,
        LIGHT: OutputPin,
    {
        pcd8544.set_x_position(0)?;
        pcd8544.set_y_position(0)?;
        for row in self.framebuffer.iter() {
            for byte in row.iter() {
                pcd8544.write_data(*byte)?;
            }
        }
        Ok(())
    }
}

impl DrawTarget<BinaryColor> for PCD8544EmbeddedGraphics {
    type Error = ();

    fn draw_pixel(&mut self, pixel: Pixel<BinaryColor>) -> core::result::Result<(), ()> {
        let Pixel(coord, color) = pixel;
        if let Ok((x @ 0..=MAX_X, y @ 0..=MAX_Y)) = coord.try_into() {
            let byte: &mut u8 = &mut self.framebuffer[(y / 8) as usize][x as usize];
            let mask: u8 = 1 << (y % 8);
            match color {
                BinaryColor::On => *byte |= mask,
                BinaryColor::Off => *byte &= !mask,
            };
        }
        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }

    fn clear(&mut self, color: BinaryColor) -> Result<(), ()> {
        let byte: u8 = match color {
            BinaryColor::On => 0xff,
            BinaryColor::Off => 0x00,
        };
        for row in self.framebuffer.iter_mut() {
            for pos in row.iter_mut() {
                *pos = byte;
            }
        }
        Ok(())
    }
}
