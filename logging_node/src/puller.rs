use crate::characteristics::{Characteristic, TemperatureHumidity, PM};
use byteorder::{BigEndian, ByteOrder};
use std::convert::From;
use std::io;
use std::net;
use std::time;

pub struct Puller {
    socket: net::UdpSocket,
}

#[derive(Debug, Clone, Copy)]
pub enum QueryCharacteristic {
    PM = 1,
    TemperatureHumidity = 2,
}

enum QueryCommand {
    GetCurrent,
    GetRecorded(time::SystemTime, QueryCharacteristic),
    GetRecordedBoundaries(QueryCharacteristic),
}

impl QueryCommand {
    fn encode(self) -> Vec<u8> {
        match self {
            QueryCommand::GetCurrent => vec![1],
            QueryCommand::GetRecorded(time, ch) => {
                let mut buf = vec![0; 5];
                buf[0] = 2;
                BigEndian::write_u32(
                    &mut buf[1..],
                    time.duration_since(time::UNIX_EPOCH).unwrap().as_secs() as u32,
                );
                buf[4] = ch as u8;
                buf
            }
            QueryCommand::GetRecordedBoundaries(ch) => {
                let mut buf = vec![0; 2];
                buf[0] = 3;
                buf[1] = ch as u8;
                buf
            }
        }
    }
}

enum ResponseType {
    Current = 1,
    Recorded = 2,
    Boundaries = 3,
}

const READ_TIMEOUT_SECS: u64 = 5;
const RECV_BUFFER_SIZE: usize = 64;

#[derive(Debug, Clone)]
pub struct CharacteristicDecodeError;

#[derive(Debug)]
pub struct Boundaries {
    characteristic: QueryCharacteristic,
    last_sample_at: time::SystemTime,
    num_samples: u16,
}

pub trait NetworkedCharacteristic {
    fn decode(input: &[u8]) -> Result<Self, CharacteristicDecodeError>
    where
        Self: Characteristic + std::marker::Sized;
    fn query_characteristic() -> QueryCharacteristic;
}

impl NetworkedCharacteristic for TemperatureHumidity {
    fn decode(input: &[u8]) -> Result<Self, CharacteristicDecodeError> {
        if input.len() != 4 {
            return Err(CharacteristicDecodeError);
        }
        Ok(TemperatureHumidity {
            temperature: BigEndian::read_i16(&input[0..]),
            humidity: BigEndian::read_i16(&input[2..]),
        })
    }

    fn query_characteristic() -> QueryCharacteristic {
        QueryCharacteristic::TemperatureHumidity
    }
}

impl NetworkedCharacteristic for PM {
    fn decode(input: &[u8]) -> Result<Self, CharacteristicDecodeError> {
        if input.len() != 4 {
            return Err(CharacteristicDecodeError);
        }
        Ok(PM {
            pm2_5: BigEndian::read_i16(&input[0..]),
            pm10: BigEndian::read_i16(&input[2..]),
        })
    }

    fn query_characteristic() -> QueryCharacteristic {
        QueryCharacteristic::PM
    }
}

enum ResponseVerification {
    Valid,
    Invalid,
}

impl From<bool> for ResponseVerification {
    fn from(b: bool) -> Self {
        if b {
            ResponseVerification::Valid
        } else {
            ResponseVerification::Invalid
        }
    }
}

#[derive(Debug)]
pub enum PullerError {
    Timeout,
    SocketError(io::Error),
    CharacteristicDecodeError,
}

impl From<CharacteristicDecodeError> for PullerError {
    fn from(_err: CharacteristicDecodeError) -> Self {
        PullerError::CharacteristicDecodeError
    }
}

impl From<io::Error> for PullerError {
    fn from(err: io::Error) -> PullerError {
        PullerError::SocketError(err)
    }
}

pub struct AllCharacteristics {
    pub temperature_humidity: TemperatureHumidity,
    pub pm: PM,
}

impl Puller {
    pub fn new(address: impl net::ToSocketAddrs) -> Self {
        let socket = net::UdpSocket::bind(address).unwrap();
        socket
            .set_read_timeout(Some(time::Duration::from_secs(READ_TIMEOUT_SECS)))
            .unwrap();
        Puller { socket: socket }
    }

    fn query(&self, command: QueryCommand) -> Result<(), io::Error> {
        self.socket.send(&command.encode())?;
        Ok(())
    }

    fn wait_for_response<F, R>(&self, verify: F) -> Result<Vec<u8>, PullerError>
    where
        F: Fn(&[u8]) -> R,
        R: Into<ResponseVerification>,
    {
        let mut buffer: Vec<u8> = vec![0; RECV_BUFFER_SIZE];

        loop {
            match self.socket.recv(&mut buffer) {
                Ok(read_size) => {
                    buffer.truncate(read_size);
                    match verify(&buffer).into() {
                        ResponseVerification::Valid => break Ok(buffer),
                        ResponseVerification::Invalid => continue,
                    }
                }
                Err(e) => match e.kind() {
                    io::ErrorKind::WouldBlock => break Err(PullerError::Timeout),
                    _ => break Err(PullerError::SocketError(e)),
                },
            }
        }
    }

    /// Retrieve latest known values of all characteristics supported
    /// by device.
    pub fn get_current(&self) -> Result<AllCharacteristics, PullerError> {
        self.query(QueryCommand::GetCurrent)?;
        let response = self
            .wait_for_response(|resp| resp[0] == ResponseType::Current as u8 && resp.len() == 9)?;
        Ok(AllCharacteristics {
            temperature_humidity: NetworkedCharacteristic::decode(&response[1..])?,
            pm: NetworkedCharacteristic::decode(&response[5..])?,
        })
    }

    /// Retrieve recorded characteristic at specified time. This
    /// method is polymorphic to returned characteristic and chooses
    /// what to request based on it.
    pub fn get_recorded<C: Characteristic + NetworkedCharacteristic>(
        &self,
        time: time::SystemTime,
    ) -> Result<C, PullerError> {
        self.query(QueryCommand::GetRecorded(
            time,
            <C as NetworkedCharacteristic>::query_characteristic(),
        ))?;
        let response = self
            .wait_for_response(|resp| resp[0] == ResponseType::Recorded as u8 && resp.len() == 5)?;
        Ok(NetworkedCharacteristic::decode(&response[1..])?)
    }

    /// Retrieve time interval of samples stored on device.
    pub fn get_boundaries(
        &self,
        characteristic: QueryCharacteristic,
    ) -> Result<Boundaries, PullerError> {
        self.query(QueryCommand::GetRecordedBoundaries(characteristic))?;
        let response = self.wait_for_response(|resp| {
            resp[0] == ResponseType::Boundaries as u8
                && resp[1] == characteristic as u8
                && resp.len() == 8
        })?;
        Ok(Boundaries {
            characteristic: characteristic,
            last_sample_at: time::UNIX_EPOCH
                + time::Duration::from_secs(BigEndian::read_u32(&response[2..]) as u64),
            num_samples: BigEndian::read_u16(&response[6..]),
        })
    }
}
