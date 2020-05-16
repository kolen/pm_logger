use core::result::Result;
use embedded_hal::serial::{Read, Write};
use embedded_hal::timer;

pub enum ParseError<E> {
    Incomplete,
    Error(E),
}

pub type ParseResult<M, E> = Result<M, ParseError<E>>;
pub enum ResponseError<E> {
    Timeout,
    SerialError(E),
}

pub struct RequestResponse<R: Read<u8>, W: Write<u8>> {
    serial_read: R,
    serial_write: W,
}

impl<R: Read<u8>, W: Write<u8>> RequestResponse<R, W> {
    /// Reads parsed message from `rx`
    ///
    /// Reads parsed message from `rx` side of serial, using parse
    /// function using external `buffer` when waiting for more
    /// data. Buffer must be large enough to fit message of maximum
    /// length you want to parse. Set `buffer_pos` to zero at the
    /// beginning of operation; as you will call it continuosly due to
    /// nature of `nb`, `buffer` and `buffer_pos` will change and be
    pub fn get_message<M, E, P, T>(
        &mut self,
        parse: P,
        buffer: &mut [u8],
        buffer_pos: &mut usize,
        timer: &mut T
    ) -> nb::Result<M, ResponseError<R::Error>>
    where
        P: Fn(&[u8]) -> ParseResult<M, E>,
        T: timer::CountDown
    {
        loop {
            timer.wait().map_err(|e| e.map(|_| ResponseError::Timeout))?;
            buffer[*buffer_pos] = self
                .serial_read
                .read()
                .map_err(|e| e.map(|ei| ResponseError::SerialError(ei)))?;
            let parse_result = parse(&buffer[0..=*buffer_pos]);
            match parse_result {
                Ok(m) => return Ok(m),
                Err(ParseError::Incomplete) => {
                    *buffer_pos += 1;
                    continue;
                }
                Err(ParseError::Error(_)) => {
                    *buffer_pos = 0;
                    continue;
                }
            }
        }
    }
}
