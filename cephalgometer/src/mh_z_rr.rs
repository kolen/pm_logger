use crate::mh_z19_packet_iter::PacketIter;
use crate::request_response;
use embedded_hal::serial;
use embedded_hal::timer;
use mh_z19;

const PACKET_LENGTH: usize = 9; // Can't read get from mh_z19::Packet type btw

#[allow(non_camel_case_types)]
struct MH_Z_RR<R, W, TF, T>
where
    R: serial::Read<u8>,
    W: serial::Write<u8>,
    TF: Fn() -> T,
    T: timer::CountDown,
{
    serial_read: R,
    serial_write: W,
    timeout_factory: TF,
}

// Arrays can't be converted into iterator for now, see
// https://github.com/rust-lang/rust/issues/65798

struct ReadConcentrationProgress<'a, R, W, TF, T>
where
    R: serial::Read<u8>,
    W: serial::Write<u8>,
    TF: Fn() -> T,
    T: timer::CountDown,
{
    mh_z_rr: &'a mut MH_Z_RR<R, W, TF, T>,
    msg_iter: PacketIter,
    buffer: mh_z19::Packet,
    buffer_pos: usize,
    query_state: request_response::QueryState,
    timeout: T,
}

impl<'a, R, W, TF, T> ReadConcentrationProgress<'a, R, W, TF, T>
where
    R: serial::Read<u8>,
    W: serial::Write<u8>,
    TF: Fn() -> T,
    T: timer::CountDown,
{
    pub fn run(
        &mut self,
    ) -> nb::Result<u32, request_response::Error<request_response::SerialError<R::Error, W::Error>>>
    {
        fn parse(data: &[u8]) -> request_response::ParseResult<u32, mh_z19::MHZ19Error> {
            if data.len() < PACKET_LENGTH {
                Err(request_response::ParseError::Incomplete)
            } else {
                mh_z19::parse_gas_contentration_ppm(data)
                    .map_err(|e| request_response::ParseError::Error(e))
            }
        };
        request_response::query(
            &mut self.mh_z_rr.serial_read,
            &mut self.mh_z_rr.serial_write,
            &mut self.msg_iter,
            &mut self.timeout,
            &mut parse,
            &mut self.buffer,
            &mut self.buffer_pos,
            &mut self.query_state,
        )
    }
}

impl<R, W, TF, T> MH_Z_RR<R, W, TF, T>
where
    R: serial::Read<u8>,
    W: serial::Write<u8>,
    TF: Fn() -> T,
    T: timer::CountDown,
{
    pub fn new(serial_read: R, serial_write: W, timeout_factory: TF) -> Self
    where
        R: serial::Read<u8>,
        W: serial::Write<u8>,
    {
        MH_Z_RR {
            serial_read,
            serial_write,
            timeout_factory,
        }
    }

    pub fn read_gas_concentration(
        &mut self,
        device_number: u8,
    ) -> ReadConcentrationProgress<R, W, TF, T> {
        let msg_iter = mh_z19::read_gas_concentration(device_number).into();
        let timeout = (self.timeout_factory)();
        ReadConcentrationProgress {
            mh_z_rr: self,
            msg_iter,
            buffer: [0; 9],
            buffer_pos: 0,
            query_state: request_response::QueryState::Writing,
            timeout,
        }
    }
}
