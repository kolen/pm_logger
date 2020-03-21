//! A request-response framework for
//! [`embedded-hal`](https://crates.io/crates/embedded-hal) serial
//! connections.
//!
//! # Examples
//!
//!

#![no_std]

use embedded_hal::serial;
use embedded_hal::timer::CountDown;
use nb;

enum ResponseError<SE> {
    Timeout,
    FrameInvalid(ParseStatus),
    SerialError(SE)
}

// TODO: check if we can use nom and don't reinvent the wheel
enum ParseStatus {
    /// Everything is ok, just more data needed
    Incomplete,
    /// Bad, unparsable data/frame detected
    Bad,
    /// Data/frame looks good at first sight, but failed verification
    /// (i.e. checksum)
    Damaged,
    /// Unrelated data/frame (i.e. response to unrelated request)
    Unrelated
}

/// Parser type. Unlike general-purpose parser combinator libraries
/// (i.e. nom), we don't have 'remaining' in successful output: all
/// remaining data is still unread. This library does not support
/// look-ahead.
type Parser<O> = FnOnce(&[u8]) -> Error<O, ParseStatus>;
type Filter = Parser<&[u8]>;

pub struct SerialRR<RX, TX, CD, T, M>
where
    RX: serial::Read<u8>,
    TX: serial::Write<u8>,
    CD: CountDown,
    T: FnMut() -> CD,
{
    rx: RX,
    tx: TX,
    make_timer: T
}

impl<RX, TX, CD, T, M> SerialRR<RX, TX, CD, T, M>
where
    RX: serial::Read<u8>,
    TX: serial::Write<u8>,
    CD: CountDown,
    T: FnMut() -> CD,
    M: Matcher,
{
    /// Creates a new `SerialRR` instance. `make_timer` is a function
    /// that makes `embedded_hal::timer::CountDown` timers (because
    /// they are single-use) that are used for timeout.
    pub fn new(rx: RX, tx: TX, make_timer: T) -> Self {
        SerialRR {
            rx,
            tx,
            make_timer,
        }
    }

    /// Runs query cycle
    pub fn query(
        self,
        request: &[u8],
        parser: Parser
    ) -> nb::Result<&[u8], ResponseError> {
        self.tx.bwrite_all(request);
        self.timer.start();
        loop {
            match wait_frame_start() {
                Ok() => break
            }
            match self.rx.read() {
                Ok()
                Err(nb::Error::WouldBlock) => continue;
            }
        }
        Ok(todo!())
    }

    fn run_parser(self, parser: Parser,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
