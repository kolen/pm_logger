use super::{Address, MeasurementStoreMemory};

struct MeasurementStoreMemoryRAM {
    data: Vec<u8>
}

impl MeasurementStoreMemory for MeasurementStoreMemoryRAM {
    type Error = ();

    fn read_page(
        &self,
        address: Address,
        buffer: &mut [u8],
        length: u16,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn write_page(&self, address: Address, buffer: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }

    fn erase_sector(&self, address: Address) -> Result<(), Self::Error> {
        todo!()
    }
}
