//! MH-Zxx request-response wrapper over `mh-z19` crate

use embedded_hal::serial::{Read, Write};
use embedded_hal::timer::CountDown;
use nom;
use mh_z19;

#[allow(non_camel_case_types)]
pub struct MH_Zxx<TX, RX, CD, T>
where
    TX: Write<u8>,
    RX: Read<u8>,
    CD: CountDown,
    T: FnMut() -> CD,
{
    tx: TX,
    rx: RX,
    timeout: T,
}

impl<TX, RX, CD, T> MH_Zxx<TX, RX, CD, T>
where
    TX: Write<u8>,
    RX: Read<u8>,
    CD: CountDown,
    T: FnMut() -> CD,
{
    pub fn new(rx: RX, tx: TX, timeout: T) -> Self {
        MH_Zxx { tx, rx, timeout }
    }
}
