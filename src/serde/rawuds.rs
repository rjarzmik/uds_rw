use super::Payload;
use crate::proto::rawuds::RawUds;
use crate::UdsError::{self};
use std::io::{Read, Write};

impl Payload for RawUds {
    fn length(&self) -> usize {
        self.data.len()
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, UdsError> {
        super::default_read(reader, payload_length)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), UdsError> {
        self.data.resize(1 + payload_length, 0u8);
        // The first byte is supposed to hold the SID, and should have been
        // filled at initialization
        reader.read_exact(&mut self.data[1..])?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        writer.write_all(&self.data)?;
        Ok(())
    }
}
