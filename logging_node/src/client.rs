use crate::characteristics::{Characteristic, TemperatureHumidity, PM};
use byteorder::{BigEndian, ByteOrder};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use failure::Error;
use std::convert::From;
use std::io;
use std::net;
use std::ops::RangeInclusive;

pub struct Client {
    socket: net::UdpSocket,
}

#[derive(Debug, Clone, Copy)]
pub enum QueryCharacteristic {
    PM = 1,
    TemperatureHumidity = 2,
}

enum QueryCommand {
    GetCurrent,
    GetRecorded(DateTime<Utc>, QueryCharacteristic),
    GetRecordedBoundaries(QueryCharacteristic),
}

impl QueryCommand {
    fn encode(self) -> Vec<u8> {
        match self {
            QueryCommand::GetCurrent => vec![1],
            QueryCommand::GetRecorded(time, ch) => {
                let mut buf = vec![0; 6];
                buf[0] = 2;
                BigEndian::write_u32(&mut buf[1..], time.timestamp() as u32);
                buf[5] = ch as u8;
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

const READ_TIMEOUT: i64 = 5;
const RECV_BUFFER_SIZE: usize = 64;

#[derive(Debug, Clone)]
pub struct CharacteristicDecodeError;

#[derive(Debug)]
pub struct Boundaries {
    pub characteristic: QueryCharacteristic,
    pub last_sample_at: DateTime<Utc>,
    pub num_samples: u16,
}

impl Boundaries {
    pub fn sampling_interval(&self) -> Duration {
        match self.characteristic {
            QueryCharacteristic::PM => Duration::hours(1),
            QueryCharacteristic::TemperatureHumidity => Duration::minutes(10),
        }
    }

    /// Returns sequence of times covered by this boundaries
    pub fn times(&self) -> impl Iterator<Item = DateTime<Utc>> {
        let interval = self.sampling_interval();
        (0..self.num_samples).scan(self.last_sample_at, move |time, _i| {
            let current_time = *time;
            *time = *time - interval;
            Some(current_time)
        })
    }

    pub fn date_range(&self) -> RangeInclusive<DateTime<Utc>> {
        let first_sample_at =
            self.last_sample_at - self.sampling_interval() * (self.num_samples - 1) as i32;
        first_sample_at..=self.last_sample_at
    }
}

pub trait NetworkedCharacteristic {
    /// Decodes measured characteristic value from slice of network
    /// packet, returning characteristic value or None if "no data
    /// available" is recorded for that sample.
    fn decode(input: &[u8]) -> Result<Option<Self>, CharacteristicDecodeError>
    where
        Self: Characteristic + std::marker::Sized;

    fn query_characteristic() -> QueryCharacteristic;
}

impl NetworkedCharacteristic for TemperatureHumidity {
    fn decode(input: &[u8]) -> Result<Option<Self>, CharacteristicDecodeError> {
        if input.len() != 4 {
            return Err(CharacteristicDecodeError);
        }
        let temperature = BigEndian::read_i16(&input[0..]);
        let humidity = BigEndian::read_i16(&input[2..]);

        if temperature == 0 && humidity == 0 {
            return Ok(None);
        }

        Ok(Some(TemperatureHumidity {
            temperature: temperature,
            humidity: humidity,
        }))
    }

    fn query_characteristic() -> QueryCharacteristic {
        QueryCharacteristic::TemperatureHumidity
    }
}

impl NetworkedCharacteristic for PM {
    fn decode(input: &[u8]) -> Result<Option<Self>, CharacteristicDecodeError> {
        if input.len() != 4 {
            return Err(CharacteristicDecodeError);
        }

        let pm2_5 = BigEndian::read_i16(&input[0..]);
        let pm10 = BigEndian::read_i16(&input[2..]);

        if pm2_5 == 0 && pm10 == 0 {
            return Ok(None);
        }

        Ok(Some(PM {
            pm2_5: pm2_5,
            pm10: pm10,
        }))
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

#[derive(Debug, Fail)]
pub enum ClientError {
    #[fail(display = "timeout waiting for response from device")]
    Timeout,
    #[fail(display = "socket error")] //TODO: check
    SocketError(#[fail(cause)] io::Error),
    #[fail(display = "error decoding characteristic value")]
    CharacteristicDecodeError,
}

impl From<CharacteristicDecodeError> for ClientError {
    fn from(_err: CharacteristicDecodeError) -> Self {
        ClientError::CharacteristicDecodeError
    }
}

impl From<io::Error> for ClientError {
    fn from(err: io::Error) -> ClientError {
        ClientError::SocketError(err)
    }
}

pub struct AllCharacteristics {
    pub temperature_humidity: Option<TemperatureHumidity>,
    pub pm: Option<PM>,
}

impl Client {
    pub fn new(address: impl net::ToSocketAddrs) -> Result<Self, Error> {
        let socket = net::UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.set_read_timeout(Some(Duration::seconds(READ_TIMEOUT).to_std().unwrap()))?;
        socket.connect(address)?;
        Ok(Client { socket: socket })
    }

    fn query(&self, command: QueryCommand) -> Result<(), io::Error> {
        let bytes = command.encode();
        trace!("Sending query command: {:x?}", &bytes);
        self.socket.send(&bytes)?;
        Ok(())
    }

    fn wait_for_response<F, R>(&self, verify: F) -> Result<Vec<u8>, ClientError>
    where
        F: Fn(&[u8]) -> R,
        R: Into<ResponseVerification>,
    {
        let mut buffer: Vec<u8> = vec![0; RECV_BUFFER_SIZE];

        loop {
            match self.socket.recv(&mut buffer) {
                Ok(read_size) => {
                    buffer.truncate(read_size);
                    trace!("Received packet: {:x?}", &buffer);
                    match verify(&buffer).into() {
                        ResponseVerification::Valid => break Ok(buffer),
                        ResponseVerification::Invalid => {
                            debug!("Unrelated packet received: {:?}", &buffer);
                            continue;
                        }
                    }
                }
                Err(e) => match e.kind() {
                    io::ErrorKind::WouldBlock => break Err(ClientError::Timeout),
                    _ => break Err(ClientError::SocketError(e)),
                },
            }
        }
    }

    /// Retrieve latest known values of all characteristics supported
    /// by device.
    pub fn get_current(&self) -> Result<AllCharacteristics, ClientError> {
        self.query(QueryCommand::GetCurrent)?;
        let response = self
            .wait_for_response(|resp| resp.len() == 9 && resp[0] == ResponseType::Current as u8)?;
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
        time: impl Into<DateTime<Utc>>,
    ) -> Result<Option<C>, ClientError> {
        let characteristic = <C as NetworkedCharacteristic>::query_characteristic();
        self.query(QueryCommand::GetRecorded(time.into(), characteristic))?;
        let response = self.wait_for_response(|resp| {
            resp.len() == 10
                && resp[0] == ResponseType::Recorded as u8
                && resp[5] == characteristic as u8
        })?;
        Ok(NetworkedCharacteristic::decode(&response[6..])?)
    }

    /// Retrieve time interval of samples stored on device.
    pub fn get_boundaries(
        &self,
        characteristic: QueryCharacteristic,
    ) -> Result<Boundaries, ClientError> {
        self.query(QueryCommand::GetRecordedBoundaries(characteristic))?;
        let response = self.wait_for_response(|resp| {
            resp.len() == 8
                && resp[0] == ResponseType::Boundaries as u8
                && resp[1] == characteristic as u8
        })?;
        let last_sample_ts = BigEndian::read_u32(&response[2..]) as i64;
        let last_sample_at =
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(last_sample_ts, 0), Utc);
        Ok(Boundaries {
            characteristic: characteristic,
            last_sample_at: last_sample_at,
            num_samples: BigEndian::read_u16(&response[6..]),
        })
    }
}
