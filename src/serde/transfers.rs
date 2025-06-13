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

        write_sized(
            writer,
            self.memory_address,
            self.memory_address_bytes.into(),
        )?;
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
        write_sized(
            writer,
            self.max_block_size,
            self.max_block_size_bytes.into(),
        )?;
        Ok(())
    }
}

impl From<ModeOfOperation> for u8 {
    fn from(item: ModeOfOperation) -> Self {
        match item {
            ModeOfOperation::Reserved => 0,
            ModeOfOperation::AddFile => 1,
            ModeOfOperation::DeleteFile => 2,
            ModeOfOperation::ReplaceFile => 3,
            ModeOfOperation::ReadFile => 4,
            ModeOfOperation::ReadDir => 5,
        }
    }
}

impl From<u8> for ModeOfOperation {
    fn from(item: u8) -> Self {
        match item {
            0 => Self::Reserved,
            1 => Self::AddFile,
            2 => Self::DeleteFile,
            3 => Self::ReplaceFile,
            4 => Self::ReadFile,
            5 => Self::ReadDir,
            _ => Self::Reserved,
        }
    }
}

impl RequestFileTransferReq {
    fn file_tr_req_calculate_length(
        mode: ModeOfOperation,
        path_bytes: usize,
        file_size_bytes: u8,
    ) -> usize {
        if (ModeOfOperation::AddFile == mode) || (ModeOfOperation::ReplaceFile == mode) {
            1 + 2 + path_bytes + 1 + (file_size_bytes as usize) * 2
        } else {
            1 + 2 + path_bytes
        }
    }
}

impl Payload for RequestFileTransferReq {
    fn length(&self) -> usize {
        Self::file_tr_req_calculate_length(
            self.mode_of_operation,
            self.path_name.len(),
            self.file_size_bytes,
        )
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, UdsError> {
        super::default_read(reader, payload_length)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), UdsError> {
        *self = RequestFileTransferReq::default();
        self.mode_of_operation = ModeOfOperation::from(reader.read_u8()?);
        if ModeOfOperation::Reserved == self.mode_of_operation {
            return Err(EncodingError {
                msg: String::from("Unhandled mode of operation: Reserved"),
            });
        }
        if payload_length < Self::file_tr_req_calculate_length(self.mode_of_operation, 0, 0) {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: Self::file_tr_req_calculate_length(self.mode_of_operation, 0, 0) as u32,
            });
        }

        let file_path_and_name_bytes = read_sized(reader, 2)?;
        let mut vec = vec![0u8; file_path_and_name_bytes];
        reader.read_exact(&mut vec)?;
        self.path_name = String::from_utf8(vec).map_err(|_| UdsError::EncodingError {
            msg: "File of Path name is not a valid UTF-8 string".to_string(),
        })?;

        if (ModeOfOperation::DeleteFile != self.mode_of_operation)
            && (ModeOfOperation::ReadDir != self.mode_of_operation)
        {
            let compress_encrypt = reader.read_u8()?;
            self.encryption_method = compress_encrypt & 0x0f;
            self.compression_method = compress_encrypt >> 4;
        }

        if (ModeOfOperation::DeleteFile != self.mode_of_operation)
            && (ModeOfOperation::ReadFile != self.mode_of_operation)
            && (ModeOfOperation::ReadDir != self.mode_of_operation)
        {
            self.file_size_bytes = reader.read_u8()?;
            self.file_size_uncompressed = read_sized(reader, self.file_size_bytes.into())?;
            self.file_size_compressed = read_sized(reader, self.file_size_bytes.into())?;
        }

        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        writer.write_u8(self.mode_of_operation.into())?;
        let path_bytes =
            u16::try_from(self.path_name.len()).map_err(|_| UdsError::EncodingError {
                msg: "File of Path name is bigger than 65536 bytes".to_string(),
            })?;
        write_sized(writer, path_bytes as usize, 2)?;
        writer.write_all(self.path_name.as_bytes())?;

        if (ModeOfOperation::DeleteFile != self.mode_of_operation)
            && (ModeOfOperation::ReadDir != self.mode_of_operation)
        {
            let compress_encrypt: u8 =
                (self.compression_method << 4) | (self.encryption_method & 0x0f);
            writer.write_u8(compress_encrypt)?;
        }

        if (ModeOfOperation::DeleteFile != self.mode_of_operation)
            && (ModeOfOperation::ReadFile != self.mode_of_operation)
            && (ModeOfOperation::ReadDir != self.mode_of_operation)
        {
            writer.write_u8(self.file_size_bytes)?;
            write_sized(
                writer,
                self.file_size_uncompressed,
                self.file_size_bytes.into(),
            )?;
            write_sized(
                writer,
                self.file_size_compressed,
                self.file_size_bytes.into(),
            )?;
        }
        Ok(())
    }
}

impl RequestFileTransferRsp {
    fn file_tr_rsp_calculate_length(
        mode: ModeOfOperation,
        block_size_bytes: u8,
        data_size_bytes: u16,
    ) -> usize {
        match mode {
            ModeOfOperation::DeleteFile => 1,
            ModeOfOperation::AddFile | ModeOfOperation::ReplaceFile => {
                1 + 1 + block_size_bytes as usize + 1
            }
            ModeOfOperation::ReadDir => {
                1 + 1 + block_size_bytes as usize + 1 + 2 + data_size_bytes as usize
            }
            _ => 1 + 1 + block_size_bytes as usize + 1 + 2 + (data_size_bytes as usize) * 2,
        }
    }
}

impl Payload for RequestFileTransferRsp {
    fn length(&self) -> usize {
        Self::file_tr_rsp_calculate_length(
            self.mode_of_operation,
            self.max_block_size_bytes,
            self.file_dir_data_size_bytes,
        )
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, UdsError> {
        super::default_read(reader, payload_length)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), UdsError> {
        *self = RequestFileTransferRsp::default();
        self.mode_of_operation = ModeOfOperation::from(reader.read_u8()?);
        if ModeOfOperation::Reserved == self.mode_of_operation {
            return Err(EncodingError {
                msg: String::from("Unhandled mode of operation: Reserved"),
            });
        }
        if payload_length < Self::file_tr_rsp_calculate_length(self.mode_of_operation, 0, 0) {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: Self::file_tr_rsp_calculate_length(self.mode_of_operation, 0, 0) as u32,
            });
        }

        if ModeOfOperation::DeleteFile != self.mode_of_operation {
            self.max_block_size_bytes = reader.read_u8()?;
            self.max_block_size = read_sized(reader, self.max_block_size_bytes.into())?;
            let compress_encrypt = reader.read_u8()?;
            self.encryption_method = compress_encrypt & 0x0f;
            self.compression_method = compress_encrypt >> 4;
        }
        match self.mode_of_operation {
            ModeOfOperation::ReadDir => {
                let size_bytes = read_sized(reader, 2)? as u16;
                self.file_dir_data_size_bytes = size_bytes;
                self.file_dir_size_uncompressed = read_sized(reader, size_bytes)?;
            }
            ModeOfOperation::ReadFile => {
                let size_bytes = read_sized(reader, 2)? as u16;
                self.file_dir_data_size_bytes = size_bytes;
                self.file_dir_size_uncompressed = read_sized(reader, size_bytes)?;
                self.file_size_compressed = read_sized(reader, size_bytes)?;
            }
            _ => {}
        };
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        writer.write_u8(self.mode_of_operation.into())?;
        if ModeOfOperation::DeleteFile != self.mode_of_operation {
            writer.write_u8(self.max_block_size_bytes)?;
            write_sized(
                writer,
                self.max_block_size,
                self.max_block_size_bytes.into(),
            )?;
            let compress_encrypt: u8 =
                (self.compression_method << 4) | (self.encryption_method & 0x0f);
            writer.write_u8(compress_encrypt)?;
        }
        let size_bytes = self.file_dir_data_size_bytes;
        match self.mode_of_operation {
            ModeOfOperation::ReadDir => {
                write_sized(writer, size_bytes.into(), 2)?;
                write_sized(writer, self.file_dir_size_uncompressed, size_bytes)?;
            }
            ModeOfOperation::ReadFile => {
                write_sized(writer, size_bytes.into(), 2)?;
                write_sized(writer, self.file_dir_size_uncompressed, size_bytes)?;
                write_sized(writer, self.file_size_compressed, size_bytes)?;
            }
            _ => {}
        };
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
