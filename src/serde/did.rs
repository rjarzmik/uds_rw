use super::Payload;
use crate::proto::did::{ReadDIDReq, ReadDIDRsp, WriteDIDReq, WriteDIDRsp};
use crate::UdsError::{self, PayloadLengthTooShort};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

impl Payload for ReadDIDReq {
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
        self.did = reader.read_u16::<BigEndian>()?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        writer.write_u16::<BigEndian>(self.did)?;
        Ok(())
    }
}

impl Payload for ReadDIDRsp {
    fn length(&self) -> usize {
        2 + self.user_data.len()
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, UdsError> {
        super::default_read(reader, payload_length)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), UdsError> {
        if payload_length < 2 {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: 1u32,
            });
        }
        self.did = reader.read_u16::<BigEndian>()?;
        self.user_data.resize(payload_length - 2, 0u8);
        reader.read_exact(&mut self.user_data)?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        writer.write_u16::<BigEndian>(self.did)?;
        writer.write_all(&self.user_data)?;
        Ok(())
    }
}

impl Payload for WriteDIDReq {
    fn length(&self) -> usize {
        2 + self.user_data.len()
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
        self.did = reader.read_u16::<BigEndian>()?;
        self.user_data.resize(payload_length - 2, 0u8);
        reader.read_exact(&mut self.user_data)?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        writer.write_u16::<BigEndian>(self.did)?;
        writer.write_all(&self.user_data)?;
        Ok(())
    }
}

impl Payload for WriteDIDRsp {
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
        self.did = reader.read_u16::<BigEndian>()?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        writer.write_u16::<BigEndian>(self.did)?;
        Ok(())
    }
}
