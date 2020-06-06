#![no_std]
use core::result::Result;
use embedded_hal::serial::{Read, Write};
use embedded_hal::timer;

pub enum ParseError<E> {
    Incomplete,
    Error(E),
}

pub type ParseResult<M, E> = Result<M, ParseError<E>>;
#[derive(Debug)]
pub enum Error<E> {
    Timeout,
    SerialError(E),
}

impl<E> Error<E> {
    pub fn map<O, F>(self, op: F) -> Error<O>
    where
        F: FnOnce(E) -> O,
    {
        match self {
            Error::Timeout => Error::Timeout,
            Error::SerialError(se) => Error::SerialError(op(se)),
        }
    }
}

fn check_timeout<T, E>(timer: &mut T) -> nb::Result<(), Error<E>>
where
    T: timer::CountDown,
{
    match timer.wait() {
        Ok(()) => Err(nb::Error::Other(Error::Timeout)),
        Err(nb::Error::WouldBlock) => Ok(()),
        Err(_) => panic!("Timer error"),
    }
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
    parse: &mut P,
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
        check_timeout(timeout)?;
        if let Ok(incoming_byte) = serial_read.read() {
            buffer[*buffer_pos] = incoming_byte;
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
        } else {
            // Serial errors are ignored. Usually serial errors are:
            // 'framing error', 'overflow', etc, and are equivalent to
            // reading garbage data. This ignore behavior is almost
            // mandatory in order to make these errors recoverable.
            continue;
        }
    }
}

pub fn send_message<W, I, T>(
    serial_write: &mut W,
    message_outputter: &mut I,
    timeout: &mut T,
) -> nb::Result<(), Error<W::Error>>
where
    W: Write<u8>,
    I: Iterator<Item = u8>,
    T: timer::CountDown,
{
    for char in message_outputter {
        check_timeout(timeout)?;
        // TODO: remove block
        nb::block!(serial_write.write(char)).map_err(|e| Error::SerialError(e))?;
    }
    Ok(())
}

#[derive(Debug)]
pub enum SerialError<R, W> {
    ReadError(R),
    WriteError(W),
}

pub enum QueryState {
    Writing,
    Flushing,
    Reading,
    Completed,
}

/// Perform query, sending command and reading response
///
/// Perform query, sending command, waiting for, reading and parsing
/// response with timeout. As this function is intended to be called
/// continuously until non-`nb::WouldBlock` result, state is
/// externalized to `message_outputter` (iterator state), `buffer`,
/// `buffer_pos` and `query_state`: it's caller's responsibility to
/// hold this state.
pub fn query<R, W, M, E, P, I, T>(
    serial_read: &mut R,
    serial_write: &mut W,
    message_outputter: &mut I,
    timeout: &mut T,
    parse: &mut P,
    buffer: &mut [u8],
    buffer_pos: &mut usize,
    query_state: &mut QueryState,
) -> nb::Result<M, Error<SerialError<R::Error, W::Error>>>
where
    R: Read<u8>,
    W: Write<u8>,
    P: Fn(&[u8]) -> ParseResult<M, E>,
    I: Iterator<Item = u8>,
    T: timer::CountDown,
{
    loop {
        match *query_state {
            QueryState::Writing => {
                send_message(serial_write, message_outputter, timeout)
                    .map_err(|e| e.map(|ie| ie.map(|iie| SerialError::WriteError(iie))))?;
                *query_state = QueryState::Reading;
            }
            // For some reason, flush does not work and just
            // interferes with reading, causing
            // `ReadError(Overrun)`. Disabled for now. Probably not
            // needed for stm32f103.
            QueryState::Flushing => {
                if matches!(serial_write.flush(), Ok(())) {
                    *query_state = QueryState::Reading;
                }
            }
            QueryState::Reading => {
                let msg = get_message(serial_read, parse, buffer, buffer_pos, timeout)
                    .map_err(|e| e.map(|ie| ie.map(|iie| SerialError::ReadError(iie))))?;
                *query_state = QueryState::Completed;
                return Ok(msg);
            }
            QueryState::Completed => {
                // TODO: consider something other
                panic!("Query already completed");
            }
        }
    }
}
