use crate::characteristics::{Characteristic, TemperatureHumidity, PM};
use byteorder::{BigEndian, ByteOrder};
use std::convert::From;
use std::io;
use std::net;
use std::time;

pub struct Puller {
    socket: net::UdpSocket,
}

enum QueryCommand {
    GetCurrent = 1,
    GetRecorded = 2,
    GetRecordedBoundaries = 3,
}

enum ResponseType {
    Current = 1,
    Recorded = 2,
    Boundaries = 3,
}

const READ_TIMEOUT_SECS: u64 = 5;
const RECV_BUFFER_SIZE: usize = 64;

#[derive(Debug, Clone)]
struct CharacteristicDecodeError;

trait NetworkedCharacteristic {
    fn decode(input: &[u8]) -> Result<Self, CharacteristicDecodeError>
    where
        Self: Characteristic + std::marker::Sized;
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

enum PullerError {
    Timeout,
    SocketError(io::Error),
}

impl Puller {
    pub fn new(address: impl net::ToSocketAddrs) -> Self {
        let socket = net::UdpSocket::bind(address).unwrap();
        socket.set_read_timeout(Some(time::Duration::from_secs(READ_TIMEOUT_SECS)));
        Puller { socket: socket }
    }

    fn query(&self, command: QueryCommand) {
        self.socket.send(&[command as u8]);
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

    pub fn get_current<C: Characteristic + NetworkedCharacteristic>(&self) -> Result<C, PullerError> {
        self.query(QueryCommand::GetCurrent);
        let response = self.wait_for_response(|resp| {
            resp[0] == ResponseType::Current as u8 && resp.len() == 5
        })?;
        Ok(C::decode(&response[1..]).unwrap())
    }

    // pub fn get_recorded<C: Characteristic + NetworkedCharacteristic>(&self) -> C {}
}
