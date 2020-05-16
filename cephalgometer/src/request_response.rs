use core::result::Result;
use embedded_hal::serial::{Read, Write};
use embedded_hal::timer;

pub enum ParseError<E> {
    Incomplete,
    Error(E),
}

pub type ParseResult<M, E> = Result<M, ParseError<E>>;
pub enum Error<E> {
    Timeout,
    SerialError(E),
}

/// Reads parsed message from `rx`
///
/// Reads parsed message from `rx` side of serial, using parse
/// function using external `buffer` when waiting for more
/// data. Buffer must be large enough to fit message of maximum
/// length you want to parse. Set `buffer_pos` to zero at the
/// beginning of operation; as you will call it continuosly due to
/// nature of `nb`, `buffer` and `buffer_pos` will change and be
pub fn get_message<R, M, E, P, T>(
    serial_read: &mut R,
    parse: P,
    buffer: &mut [u8],
    buffer_pos: &mut usize,
    timeout: &mut T,
) -> nb::Result<M, Error<R::Error>>
where
    R: Read<u8>,
    P: Fn(&[u8]) -> ParseResult<M, E>,
    T: timer::CountDown,
{
    loop {
        timeout.wait().map_err(|e| e.map(|_| Error::Timeout))?;
        buffer[*buffer_pos] = serial_read
            .read()
            .map_err(|e| e.map(|ei| Error::SerialError(ei)))?;
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

pub fn send_message<W, I, T>(
    serial_write: &mut W,
    message_outputter: I,
    timeout: &mut T,
) -> nb::Result<(), Error<W::Error>>
where
    W: Write<u8>,
    I: Iterator<Item = u8>,
    T: timer::CountDown,
{
    for char in message_outputter {
        timeout.wait().map_err(|e| e.map(|_| Error::Timeout))?;
        serial_write
            .write(char)
            .map_err(|e| e.map(|ei| Error::SerialError(ei)))?;
    }
    Ok(())
}
