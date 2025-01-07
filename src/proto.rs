use crate::UdsError;
use std::io::{Read, Write};

pub mod did;
pub mod dtc;
pub mod nrc;
pub mod rawuds;
pub mod transfers;

pub trait Payload {
    fn length(&self) -> usize;
    #[allow(dead_code)]
    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, UdsError>
    where
        Self: Sized;
    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), UdsError>;
    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError>;
}

#[repr(u8)]
pub enum Sid {
    Nrc,
    RawUds(u8),
    ReadDTC,
    ReadDID,
    RequestDownload,
    TransferData,
    TransferExit,
    WriteDID,
}
