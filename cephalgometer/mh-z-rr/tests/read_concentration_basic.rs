use embedded_hal::serial;
use embedded_hal::timer;
use mh_z_rr::MH_Z_RR;
use std::sync::{Arc, Mutex};
use void::Void;

const FAKE_RESULT: [u8; 9] = [0xff, 0x01, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00, 0x79];

struct FakeMH {
    received: [u8; 16],
    sent_pos: usize,
    received_pos: usize,
    flushed: bool,
}

struct FakeMHRead {
    fake_mh: Arc<Mutex<FakeMH>>,
}

struct FakeMHWrite {
    fake_mh: Arc<Mutex<FakeMH>>,
}

struct FakeTimer {}

impl timer::CountDown for FakeTimer {
    type Time = ();

    fn start<T>(&mut self, _: T)
    where
        T: std::convert::Into<()>,
    {
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        Err(nb::Error::WouldBlock)
    }
}

impl FakeMH {
    fn new() -> Self {
        FakeMH {
            received: [0; 16],
            sent_pos: 0,
            received_pos: 0,
            flushed: false,
        }
    }

    fn into_pair(self) -> (FakeMHRead, FakeMHWrite) {
        let mh_r = Arc::new(Mutex::new(self));
        let mh_w = mh_r.clone();
        (FakeMHRead { fake_mh: mh_r }, FakeMHWrite { fake_mh: mh_w })
    }
}

impl serial::Write<u8> for FakeMHWrite {
    type Error = ();

    fn write(&mut self, char: u8) -> std::result::Result<(), nb::Error<()>> {
        let mut lock = self.fake_mh.lock().unwrap();
        let received_pos = lock.received_pos;
        lock.received[received_pos] = char;
        lock.received_pos += 1;
        Ok(())
    }

    fn flush(&mut self) -> std::result::Result<(), nb::Error<()>> {
        let mut lock = self.fake_mh.lock().unwrap();
        lock.flushed = true;
        Ok(())
    }
}

impl serial::Read<u8> for FakeMHRead {
    type Error = ();
    fn read(&mut self) -> std::result::Result<u8, nb::Error<()>> {
        let mut lock = self.fake_mh.lock().unwrap();
        if !lock.flushed {
            panic!("Reading when read has not been flushed")
        }
        let char = FAKE_RESULT[lock.sent_pos];
        lock.sent_pos += 1;
        Ok(char)
    }
}

#[test]
fn test_basic() {
    let (fake_r, fake_w) = FakeMH::new().into_pair();
    let mut mh = MH_Z_RR::new(fake_r, fake_w);
    let mut fake_timer = FakeTimer {};
    let mut conc_progress = mh.read_gas_concentration(1, &mut fake_timer);
    let conc = nb::block!(conc_progress.run());
    dbg!(conc);
}
