use super::Payload;
use crate::proto::nrc::Nrc;
use crate::UdsError::{self, *};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

impl Payload for Nrc {
    fn length(&self) -> usize {
        2
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, UdsError> {
        super::default_read(reader, payload_length)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), UdsError> {
        if payload_length != 2 {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: 2u32,
            });
        }
        self.sid = reader.read_u8()?;
        self.nrc = reader.read_u8()?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        writer.write_u8(self.sid)?;
        writer.write_u8(self.nrc)?;
        Ok(())
    }
}
