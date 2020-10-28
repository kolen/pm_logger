use measurements_store::{
    Address, MeasurementStoreMemory, MeasurementsStore, MEMORY_PAGE_SIZE, MEMORY_SIZE,
};

struct MeasurementStoreMemoryRAM {
    pub data: Vec<u8>,
}

impl MeasurementStoreMemoryRAM {
    pub fn new() -> Self {
        MeasurementStoreMemoryRAM {
            data: vec![0xff; MEMORY_SIZE as usize],
        }
    }
}

impl MeasurementStoreMemory for MeasurementStoreMemoryRAM {
    type Error = ();

    fn read_page(
        &mut self,
        address: Address,
        buffer: &mut [u8],
        length: u16,
    ) -> Result<(), Self::Error> {
        let a = address as usize;
        buffer.copy_from_slice(&self.data[a..a + (length as usize)]);
        Ok(())
    }

    fn write_page(&mut self, address: Address, buffer: &[u8]) -> Result<(), Self::Error> {
        let a = address as usize;
        let view = &mut self.data[a..a + buffer.len()];
        view.copy_from_slice(&buffer);
        Ok(())
    }

    fn erase_sector(&mut self, address: Address) -> Result<(), Self::Error> {
        let a = address as usize;
        let view = &mut self.data[a..a + (MEMORY_PAGE_SIZE as usize)];
        for b in view {
            *b = 0xff
        }
        Ok(())
    }
}

#[test]
fn test_initial_add_measurement() {
    let mem = MeasurementStoreMemoryRAM::new();
    let store = MeasurementsStore::new();
}
