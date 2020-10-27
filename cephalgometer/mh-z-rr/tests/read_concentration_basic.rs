use embedded_hal::serial;
use embedded_hal::timer;
use mh_z_rr::MH_Z_RR;
use std::sync::{Arc, Mutex};
use void::Void;

const FAKE_RESULT: [u8; 11] = [
    // First, garbage data
    0x12, 0x00, // Actual message
    0xff, 0x86, 0x02, 0x60, 0x47, 0x00, 0x00, 0x00, 0xd1,
];

struct FakeMH {
    pub received: [u8; 16],
    sent_pos: usize,
    received_pos: usize,
    flushed: bool,
    read_cycle: u32,
    write_cycle: u32,
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
            read_cycle: 0,
            write_cycle: 0,
        }
    }

    fn into_pair_and_self(self) -> (FakeMHRead, FakeMHWrite, Arc<Mutex<FakeMH>>) {
        let mh_r = Arc::new(Mutex::new(self));
        let mh_w = mh_r.clone();
        let selff = mh_r.clone();
        (
            FakeMHRead { fake_mh: mh_r },
            FakeMHWrite { fake_mh: mh_w },
            selff,
        )
    }
}

impl serial::Write<u8> for FakeMHWrite {
    type Error = ();

    fn write(&mut self, char: u8) -> std::result::Result<(), nb::Error<()>> {
        let mut lock = self.fake_mh.lock().unwrap();

        lock.write_cycle += 1;
        if lock.write_cycle % 10 != 2 {
            return Err(nb::Error::WouldBlock);
        }

        let received_pos = lock.received_pos;
        lock.received[received_pos] = char;
        lock.received_pos += 1;
        Ok(())
    }

    fn flush(&mut self) -> std::result::Result<(), nb::Error<()>> {
        let mut lock = self.fake_mh.lock().unwrap();

        lock.write_cycle += 1;
        if lock.write_cycle % 100 != 80 {
            return Err(nb::Error::WouldBlock);
        }

        lock.flushed = true;
        Ok(())
    }
}

impl serial::Read<u8> for FakeMHRead {
    type Error = ();
    fn read(&mut self) -> std::result::Result<u8, nb::Error<()>> {
        let mut lock = self.fake_mh.lock().unwrap();

        lock.read_cycle += 1;
        if lock.read_cycle % 10 != 5 {
            return Err(nb::Error::WouldBlock);
        }

        // if !lock.flushed {
        //     panic!("Reading when read has not been flushed")
        // }
        let char = FAKE_RESULT[lock.sent_pos];
        lock.sent_pos += 1;
        Ok(char)
    }
}

#[test]
fn test_basic() {
    let (fake_r, fake_w, fmh) = FakeMH::new().into_pair_and_self();
    let mut mh = MH_Z_RR::new(fake_r, fake_w);
    let mut fake_timer = FakeTimer {};
    let mut conc_progress = mh.read_gas_concentration(1, &mut fake_timer);
    let conc = nb::block!(conc_progress.run()).unwrap();
    assert_eq!(608, conc);

    let lock = fmh.lock().unwrap();
    assert_eq!(0xff, lock.received[0]);
    assert_eq!(0x01, lock.received[1]);
    assert_eq!(0x86, lock.received[2]);
    assert_eq!(0x79, lock.received[8]);
}
