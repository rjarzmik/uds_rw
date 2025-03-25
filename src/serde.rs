use super::proto::Sid;
use crate::{
    proto::{
        did::{ReadDIDReq, ReadDIDRsp, WriteDIDReq, WriteDIDRsp},
        dtc::{ReadDTCReq, ReadDTCRsp},
        nrc::Nrc,
        rawuds::RawUds,
        transfers::*,
    },
    UdsError, UdsMessage,
};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};
mod deserializer;
mod did;
mod dtc;
mod nrc;
mod rawuds;
mod serializer;
mod transfers;

use super::proto::Payload;

pub fn uds_write<W: Write>(writer: &mut W, msg: &UdsMessage) -> Result<(), UdsError> {
    if let UdsMessage::RawUds(_) = msg {
    } else {
        writer.write_u8(msg.into())?;
    }
    msg.write(writer)
}

pub fn uds_read<R: Read>(reader: &mut R, payload_length: usize) -> Result<UdsMessage, UdsError> {
    let sid = reader.read_u8()?;
    let is_rsp = (sid & 0x40) == 0x40;
    let service: Sid = (sid & !0x40).into();
    if is_rsp {
        uds_read_rsp(reader, service, payload_length - 1)
    } else {
        uds_read_req(reader, service, payload_length - 1)
    }
}

fn uds_read_rsp<R: Read>(
    reader: &mut R,
    sid: Sid,
    payload_length: usize,
) -> Result<UdsMessage, UdsError> {
    let mut uds: UdsMessage = match sid {
        Sid::Nrc => UdsMessage::Nrc(Nrc::default()),
        Sid::RawUds(sid) => UdsMessage::RawUds(RawUds { data: vec![sid] }),
        Sid::ReadDID => UdsMessage::ReadDIDRsp(ReadDIDRsp::default()),
        Sid::ReadDTC => UdsMessage::ReadDTCRsp(ReadDTCRsp::default()),
        Sid::RequestDownload => UdsMessage::RequestDownloadRsp(RequestDownloadRsp::default()),
        Sid::TransferData => UdsMessage::TransferDataRsp(TransferDataRsp::default()),
        Sid::TransferExit => UdsMessage::TransferExitRsp(TransferExitRsp::default()),
        Sid::WriteDID => UdsMessage::WriteDIDRsp(WriteDIDRsp::default()),
    };
    uds.read_replace(reader, payload_length)?;
    Ok(uds)
}

fn uds_read_req<R: Read>(
    reader: &mut R,
    sid: Sid,
    payload_length: usize,
) -> Result<UdsMessage, UdsError> {
    let mut uds: UdsMessage = match sid {
        Sid::Nrc => UdsMessage::RawUds(RawUds::default()),
        Sid::RawUds(sid) => UdsMessage::RawUds(RawUds { data: vec![sid] }),
        Sid::ReadDID => UdsMessage::ReadDIDReq(ReadDIDReq::default()),
        Sid::ReadDTC => UdsMessage::ReadDTCReq(ReadDTCReq::default()),
        Sid::RequestDownload => UdsMessage::RequestDownloadReq(RequestDownloadReq::default()),
        Sid::TransferData => UdsMessage::TransferDataReq(TransferDataReq::default()),
        Sid::TransferExit => UdsMessage::TransferExitReq(TransferExitReq::default()),
        Sid::WriteDID => UdsMessage::WriteDIDReq(WriteDIDReq::default()),
    };
    uds.read_replace(reader, payload_length)?;
    Ok(uds)
}

pub fn default_read<T: Read, P: Payload + Default>(
    reader: &mut T,
    payload_length: usize,
) -> Result<P, UdsError> {
    let mut me = P::default();
    me.read_replace(reader, payload_length)?;
    Ok(me)
}

impl From<u8> for Sid {
    fn from(value: u8) -> Self {
        use Sid::*;
        match value {
            0x19 => ReadDTC,
            0x22 => ReadDID,
            0x2e => WriteDID,
            0x34 => RequestDownload,
            0x36 => TransferData,
            0x37 => TransferExit,
            0x3f => Nrc,
            _ => RawUds(value),
        }
    }
}

impl From<&UdsMessage> for u8 {
    fn from(value: &UdsMessage) -> Self {
        use UdsMessage::*;
        match value {
            Nrc(_) => 0x7f,
            ReadDIDReq(_) => 0x22,
            ReadDIDRsp(_) => 0x62,
            ReadDTCReq(_) => 0x19,
            ReadDTCRsp(_) => 0x59,
            RequestDownloadReq(_) => 0x34,
            RequestDownloadRsp(_) => 0x74,
            TransferDataReq(_) => 0x36,
            TransferDataRsp(_) => 0x76,
            TransferExitReq(_) => 0x37,
            TransferExitRsp(_) => 0x77,
            WriteDIDReq(_) => 0x2e,
            WriteDIDRsp(_) => 0x6e,
            RawUds(u) => u.data[0],
        }
    }
}

impl UdsMessage {
    #[allow(dead_code)]
    fn length(&self) -> usize {
        use UdsMessage::*;
        1 + match self {
            Nrc(p) => p.length(),
            RawUds(p) => p.length(),
            ReadDIDReq(p) => p.length(),
            ReadDIDRsp(p) => p.length(),
            ReadDTCReq(p) => p.length(),
            ReadDTCRsp(p) => p.length(),
            RequestDownloadReq(p) => p.length(),
            RequestDownloadRsp(p) => p.length(),
            TransferDataReq(p) => p.length(),
            TransferDataRsp(p) => p.length(),
            TransferExitReq(p) => p.length(),
            TransferExitRsp(p) => p.length(),
            WriteDIDReq(p) => p.length(),
            WriteDIDRsp(p) => p.length(),
        }
    }

    fn read_replace<R: Read>(
        &mut self,
        reader: &mut R,
        payload_length: usize,
    ) -> Result<(), UdsError> {
        use UdsMessage::*;
        match self {
            Nrc(p) => p.read_replace(reader, payload_length),
            RawUds(p) => p.read_replace(reader, payload_length),
            ReadDIDReq(p) => p.read_replace(reader, payload_length),
            ReadDIDRsp(p) => p.read_replace(reader, payload_length),
            ReadDTCReq(p) => p.read_replace(reader, payload_length),
            ReadDTCRsp(p) => p.read_replace(reader, payload_length),
            RequestDownloadReq(p) => p.read_replace(reader, payload_length),
            RequestDownloadRsp(p) => p.read_replace(reader, payload_length),
            TransferDataReq(p) => p.read_replace(reader, payload_length),
            TransferDataRsp(p) => p.read_replace(reader, payload_length),
            TransferExitReq(p) => p.read_replace(reader, payload_length),
            TransferExitRsp(p) => p.read_replace(reader, payload_length),
            WriteDIDReq(p) => p.read_replace(reader, payload_length),
            WriteDIDRsp(p) => p.read_replace(reader, payload_length),
        }
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<(), UdsError> {
        use UdsMessage::*;
        match self {
            Nrc(p) => p.write(writer),
            RawUds(p) => p.write(writer),
            ReadDIDReq(p) => p.write(writer),
            ReadDIDRsp(p) => p.write(writer),
            ReadDTCReq(p) => p.write(writer),
            ReadDTCRsp(p) => p.write(writer),
            RequestDownloadReq(p) => p.write(writer),
            RequestDownloadRsp(p) => p.write(writer),
            TransferDataReq(p) => p.write(writer),
            TransferDataRsp(p) => p.write(writer),
            TransferExitReq(p) => p.write(writer),
            TransferExitRsp(p) => p.write(writer),
            WriteDIDReq(p) => p.write(writer),
            WriteDIDRsp(p) => p.write(writer),
        }
    }
}
