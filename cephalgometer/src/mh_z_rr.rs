use crate::mh_z19_packet_iter::PacketIter;
use crate::request_response;
use embedded_hal::serial;
use embedded_hal::timer;
use mh_z19;

const PACKET_LENGTH: usize = 9; // Can't read get from mh_z19::Packet type btw

#[allow(non_camel_case_types)]
pub struct MH_Z_RR<R, W>
where
    R: serial::Read<u8>,
    W: serial::Write<u8>,
{
    serial_read: R,
    serial_write: W,
}

pub struct ReadConcentrationProgress<'a, 'b, R, W, T>
where
    R: serial::Read<u8>,
    W: serial::Write<u8>,
    T: timer::CountDown,
{
    mh_z_rr: &'a mut MH_Z_RR<R, W>,
    msg_iter: PacketIter,
    buffer: mh_z19::Packet,
    buffer_pos: usize,
    query_state: request_response::QueryState,
    timeout: &'b mut T,
}

impl<'a, 'b, R, W, T> ReadConcentrationProgress<'a, 'b, R, W, T>
where
    R: serial::Read<u8>,
    W: serial::Write<u8>,
    T: timer::CountDown,
{
    pub fn run(
        &mut self,
    ) -> nb::Result<u32, request_response::Error<request_response::SerialError<R::Error, W::Error>>>
    {
        fn parse(data: &[u8]) -> request_response::ParseResult<u32, mh_z19::MHZ19Error> {
            // mh_z19 as of 0.3.0 does not support deciding early if
            // packet is bad or more data required, so we redo parts
            // of parsing here
            match data.len() {
                1 => {
                    if data[0] == 0xff {
                        Err(request_response::ParseError::Incomplete)
                    } else {
                        Err(request_response::ParseError::Error(
                            mh_z19::MHZ19Error::WrongStartByte(data[0]),
                        ))
                    }
                }
                2 => {
                    if data[1] == 0x86 {
                        Err(request_response::ParseError::Incomplete)
                    } else {
                        Err(request_response::ParseError::Error(
                            mh_z19::MHZ19Error::WrongPacketType(0x86, data[1]),
                        ))
                    }
                }
                PACKET_LENGTH => mh_z19::parse_gas_contentration_ppm(data)
                    .map_err(|e| request_response::ParseError::Error(e)),
                _ => Err(request_response::ParseError::Incomplete),
            }
        };
        request_response::query(
            &mut self.mh_z_rr.serial_read,
            &mut self.mh_z_rr.serial_write,
            &mut self.msg_iter,
            self.timeout,
            &mut parse,
            &mut self.buffer,
            &mut self.buffer_pos,
            &mut self.query_state,
        )
    }
}

impl<R, W> MH_Z_RR<R, W>
where
    R: serial::Read<u8>,
    W: serial::Write<u8>,
{
    pub fn new(serial_read: R, serial_write: W) -> Self
    where
        R: serial::Read<u8>,
        W: serial::Write<u8>,
    {
        MH_Z_RR {
            serial_read,
            serial_write,
        }
    }

    pub fn read_gas_concentration<'a, 'b, T: timer::CountDown>(
        &'a mut self,
        device_number: u8,
        timeout: &'b mut T,
    ) -> ReadConcentrationProgress<'a, 'b, R, W, T> {
        let msg_iter = mh_z19::read_gas_concentration(device_number).into();
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
