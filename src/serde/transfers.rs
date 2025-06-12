use super::Payload;
use crate::proto::transfers::*;
use crate::UdsError::{self, *};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

fn read_sized<R: Read>(reader: &mut R, nb_bytes: u16) -> Result<usize, UdsError> {
    let mut vec = vec![0u8; nb_bytes as usize];
    reader.read_exact(&mut vec)?;
    let out: usize = match nb_bytes {
        1 => {
            let a: [u8; 1] = vec.try_into().unwrap();
            Ok(u8::from_be_bytes(a) as usize)
        }
        2 => {
            let a: [u8; 2] = vec.try_into().unwrap();
            Ok(u16::from_be_bytes(a) as usize)
        }
        4 => {
            let a: [u8; 4] = vec.try_into().unwrap();
            Ok(u32::from_be_bytes(a) as usize)
        }
        _ => Err(UdsError::UnexpectedPayloadType { value: 0 }),
    }?;
    Ok(out)
}

fn write_sized<W: Write>(writer: &mut W, val: usize, nb_bytes: u16) -> Result<(), UdsError> {
    let vec = match nb_bytes {
        1 => (val as u8).to_be_bytes().to_vec(),
        2 => (val as u16).to_be_bytes().to_vec(),
        4 => (val as u32).to_be_bytes().to_vec(),
        _ => {
            return Err(UdsError::EncodingError {
                msg: "bytes should be 1,2 or 4".to_string(),
            })
        }
    };
    writer.write_all(&vec)?;
    Ok(())
}

impl RequestDownloadReq {
    fn transfer_req_calculate_length(memory_address_bytes: u8, memory_size_bytes: u8) -> usize {
        1 + 1 + memory_address_bytes as usize + memory_size_bytes as usize
    }
}

impl Payload for RequestDownloadReq {
    fn length(&self) -> usize {
        Self::transfer_req_calculate_length(self.memory_address_bytes, self.memory_size_bytes)
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, UdsError> {
        super::default_read(reader, payload_length)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), UdsError> {
        if payload_length < Self::transfer_req_calculate_length(0, 0) {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: Self::transfer_req_calculate_length(0, 0) as u32,
            });
        }
        let compress_encrypt = reader.read_u8()?;
        self.encryption_method = compress_encrypt & 0x3f;
        self.compression_method = compress_encrypt >> 4;

        let memory_bytes = reader.read_u8()?;
        self.memory_address_bytes = memory_bytes & 0x3f;
        self.memory_size_bytes = memory_bytes >> 4;

        self.memory_address = read_sized(reader, self.memory_address_bytes.into())?;
        self.memory_size = read_sized(reader, self.memory_size_bytes.into())?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        let compress_encrypt: u8 = (self.compression_method << 4) | (self.encryption_method & 0x3f);
        writer.write_u8(compress_encrypt)?;
        let memory_bytes: u8 = (self.memory_size_bytes << 4) | (self.memory_address_bytes & 0x3f);
        writer.write_u8(memory_bytes)?;

        write_sized(writer, self.memory_address, self.memory_address_bytes.into())?;
        write_sized(writer, self.memory_size, self.memory_size_bytes.into())?;
        Ok(())
    }
}

impl RequestDownloadRsp {
    fn transfer_rsp_calculate_length(nb_bytes: u8) -> usize {
        1 + nb_bytes as usize
    }
}

impl Payload for RequestDownloadRsp {
    fn length(&self) -> usize {
        Self::transfer_rsp_calculate_length(self.max_block_size_bytes)
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, UdsError> {
        super::default_read(reader, payload_length)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), UdsError> {
        if payload_length < Self::transfer_rsp_calculate_length(0) {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: Self::transfer_rsp_calculate_length(0) as u32,
            });
        }
        self.max_block_size_bytes = reader.read_u8()?;
        self.max_block_size_bytes >>= 4;
        println!(
            "RequestDownloadRsp: max_block_size_bytes={}",
            self.max_block_size_bytes
        );
        self.max_block_size = read_sized(reader, self.max_block_size_bytes.into())?;
        println!("RequestDownloadRsp: max_block_size={}", self.max_block_size);
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        let max_block_size_bytes: u8 = self.max_block_size_bytes << 4;
        writer.write_u8(max_block_size_bytes)?;
        write_sized(writer, self.max_block_size, self.max_block_size_bytes.into())?;
        Ok(())
    }
}

impl Payload for TransferDataReq {
    fn length(&self) -> usize {
        1 + self.data.len()
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, UdsError> {
        super::default_read(reader, payload_length)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), UdsError> {
        if payload_length < 1 {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: 1u32,
            });
        }
        self.block_sequence_counter = reader.read_u8()?;
        self.data.resize(payload_length - 1, 0);
        reader.read_exact(&mut self.data)?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        writer.write_u8(self.block_sequence_counter)?;
        writer.write_all(&self.data)?;
        Ok(())
    }
}

impl Payload for TransferDataRsp {
    fn length(&self) -> usize {
        1
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, UdsError> {
        super::default_read(reader, payload_length)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), UdsError> {
        if payload_length != 1 {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: 1u32,
            });
        }
        self.block_sequence_counter = reader.read_u8()?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        writer.write_u8(self.block_sequence_counter)?;
        Ok(())
    }
}

impl Payload for TransferExitReq {
    fn length(&self) -> usize {
        self.user_data.len()
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, UdsError> {
        super::default_read(reader, payload_length)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), UdsError> {
        self.user_data.resize(payload_length, 0);
        reader.read_exact(&mut self.user_data)?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        writer.write_all(&self.user_data)?;
        Ok(())
    }
}

impl Payload for TransferExitRsp {
    fn length(&self) -> usize {
        1 + self.user_data.len()
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, UdsError> {
        super::default_read(reader, payload_length)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), UdsError> {
        self.user_data.resize(payload_length, 0);
        reader.read_exact(&mut self.user_data)?;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        writer.write_all(&self.user_data)?;
        Ok(())
    }
}
