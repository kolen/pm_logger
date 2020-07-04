pub mod memory_ram;

type Address = u32;
type Time = u32;

pub trait MeasurementStoreMemory {
    type Error;
    /// Reads 256-byte memory page. 8 lower bits of address must be 0.
    fn read_page(&self, address: Address, buffer: &mut [u8], length: u16) -> Result<(), Self::Error>;
    /// Writes 256-byte memory page. 8 lower bits of address must be 0.
    fn write_page(&self, address: Address, buffer: &[u8]) -> Result<(), Self::Error>;
    /// Erases 4 kilobyte memory sector. 12 lower bits of address must
    /// be 0.
    fn erase_sector(&self, address: Address) -> Result<(), Self::Error>;
}

// pub trait Measurement {
//     const ENCODED_SIZE: usize;
//     fn encode(&self, bytes: &mut [u8]);
//     fn decode(bytes: &[u8]) -> Self;
// }

pub struct MeasurementsStore {
    min_address: Address,
    min_time: Time,
    max_address: Address,
    max_time: Time,
}

const memory_size: Address = 0x100000;
const memory_max_addr: Address = 0x0fffff;
const memory_page_size: Address = 0x100;

impl MeasurementsStore
{
    pub fn add_measurement<E>(&mut self, memory: E, time: Time, measurement: &[u8]) -> Result<(), E::Error>
    where
        E: MeasurementStoreMemory,
    {
        let write_address = (self.max_address + memory_page_size) & memory_max_addr;

        // if address to write equals min_address (wrapped)
        //   clear page
        //   adjust min_address to first address of next page
        //   adjust min_time (read from min_address)
        // write to write_address

        // do min_time really needed? probably

        todo!();
    }

    pub fn get_measurement<E>(&self, memory: E, address: Address, measurement_buffer: &mut [u8]) -> Result<(), E::Error>
    where
        E: MeasurementStoreMemory,
    {
        let size = measurement_buffer.len() as u16;
        memory.read_page(address, measurement_buffer, size);
        Ok(())
    }

    /// Returns address of cell that either has specified time (if
    /// there's one), or the first cell having greater than given
    /// time.
    ///
    /// I.e. previous cell will have time < `time`; current cell (that
    /// is returned) will have time >= `time`.
    pub fn find_time_address<E>(&self, memory: E, time: Time) -> Result<Address, E::Error>
    where
        E: MeasurementStoreMemory,
    {
        let mut left = self.min_address;
        let mut right = if self.max_address > left {
            self.max_address
        } else {
            self.max_address + memory_size
        };

        while left <= right {
            let middle = ((left + right) / 2) & 0x1111_1100;
            let mut page = [0u8; 4];
            memory.read_page(middle & memory_max_addr, &mut page, 4)?;
            let page_time_ = page_time(&page);
            if page_time_ < time {
                left = middle + memory_page_size;
            } else if page_time_ > time {
                right = middle - memory_page_size;
            } else {
                return Ok(middle & memory_max_addr);
            }
        }

        Ok(left & memory_max_addr)
    }
}

fn page_time(page: &[u8]) -> Time {
    u32::from_be_bytes([page[0], page[1], page[2], page[3]])
}
