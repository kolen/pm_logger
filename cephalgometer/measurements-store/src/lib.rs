#![no_std]

pub type Address = u32;
pub type Time = u32;

pub trait MeasurementStoreMemory {
    type Error;
    /// Reads 256-byte memory page. 8 lower bits of address must be 0.
    fn read_page(
        &mut self,
        address: Address,
        buffer: &mut [u8],
        length: u16,
    ) -> Result<(), Self::Error>;
    /// Writes 256-byte memory page. 8 lower bits of address must be 0.
    fn write_page(&mut self, address: Address, buffer: &[u8]) -> Result<(), Self::Error>;
    /// Erases 4 kilobyte memory sector. 12 lower bits of address must
    /// be 0.
    fn erase_sector(&mut self, address: Address) -> Result<(), Self::Error>;
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

pub const MEMORY_SIZE: Address = 0x100000;
pub const MEMORY_MAX_ADDR: Address = 0x0fffff;
pub const MEMORY_PAGE_SIZE: Address = 0x100;

impl MeasurementsStore {
    pub fn new() -> Self {
        // TODO: find min_address, etc
        todo!();
    }

    pub fn add_measurement<E>(&mut self, mut memory: E, measurement: &[u8]) -> Result<(), E::Error>
    where
        E: MeasurementStoreMemory,
    {
        // if address to write equals min_address (wrapped)
        //   check if at the start of page, error otherwise
        //   clear page
        //   adjust min_address to first address of next page
        //   adjust min_time (read from min_address)
        // write to write_address

        let write_address = (self.max_address + MEMORY_PAGE_SIZE) & MEMORY_MAX_ADDR;

        if write_address == self.min_address {
            if write_address % MEMORY_PAGE_SIZE != 0 {
                panic!("Min address not at the page boundary");
            }
            memory.erase_sector(write_address)?;
            let next_page = (write_address + MEMORY_PAGE_SIZE) & MEMORY_MAX_ADDR;
            self.min_address = next_page;

            let mut time_buffer = [0u8; 4];
            memory.read_page(next_page, &mut time_buffer, 4)?;
            self.min_time = page_time(&time_buffer);
        }
        memory.write_page(write_address, measurement)?;
        Ok(())
    }

    pub fn get_measurement<E>(
        &self,
        mut memory: E,
        address: Address,
        measurement_buffer: &mut [u8],
    ) -> Result<(), E::Error>
    where
        E: MeasurementStoreMemory,
    {
        let size = measurement_buffer.len() as u16;
        memory.read_page(address, measurement_buffer, size)?;
        Ok(())
    }

    /// Returns address of cell that either has specified time (if
    /// there's one), or the first cell having greater than given
    /// time.
    ///
    /// I.e. previous cell will have time < `time`; current cell (that
    /// is returned) will have time >= `time`.
    pub fn find_time_address<E>(&self, mut memory: E, time: Time) -> Result<Address, E::Error>
    where
        E: MeasurementStoreMemory,
    {
        let mut left = self.min_address;
        let mut right = if self.max_address > left {
            self.max_address
        } else {
            self.max_address + MEMORY_SIZE
        };

        while left <= right {
            let middle = ((left + right) / 2) & 0x1111_1100;
            let mut page = [0u8; 4];
            memory.read_page(middle & MEMORY_MAX_ADDR, &mut page, 4)?;
            let page_time_ = page_time(&page);
            if page_time_ < time {
                left = middle + MEMORY_PAGE_SIZE;
            } else if page_time_ > time {
                right = middle - MEMORY_PAGE_SIZE;
            } else {
                return Ok(middle & MEMORY_MAX_ADDR);
            }
        }

        Ok(left & MEMORY_MAX_ADDR)
    }
}

fn page_time(page: &[u8]) -> Time {
    u32::from_be_bytes([page[0], page[1], page[2], page[3]])
}
