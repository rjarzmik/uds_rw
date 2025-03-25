use pretty_hex::pretty_hex;
use std::fmt::{self, Display, Formatter};

use message::NrcCode;

use crate::message;
use crate::UdsMessage;

mod dtc;

pub fn fmt(uds: &UdsMessage, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    match uds {
        UdsMessage::Nrc(d) => d.fmt(f),
        UdsMessage::RawUds(d) => d.fmt(f),
        UdsMessage::ReadDIDReq(d) => d.fmt(f),
        UdsMessage::ReadDIDRsp(d) => d.fmt(f),
        UdsMessage::ReadDTCReq(d) => d.fmt(f),
        UdsMessage::ReadDTCRsp(d) => d.fmt(f),
        UdsMessage::RequestDownloadReq(d) => d.fmt(f),
        UdsMessage::RequestDownloadRsp(d) => d.fmt(f),
        UdsMessage::TransferDataReq(d) => d.fmt(f),
        UdsMessage::TransferDataRsp(d) => d.fmt(f),
        UdsMessage::TransferExitReq(d) => d.fmt(f),
        UdsMessage::TransferExitRsp(d) => d.fmt(f),
        UdsMessage::WriteDIDReq(d) => d.fmt(f),
        UdsMessage::WriteDIDRsp(d) => d.fmt(f),
    }
}

impl Display for message::Nrc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match NrcCode::try_from(self.nrc) {
            Ok(nrc) => write!(f, "Nrc 0x{:2x} ({nrc:?})", self.nrc),
            _ => write!(f, "Nrc 0x{:2x}", self.nrc),
        }
    }
}

impl Display for message::RawUds {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "UdsBytes:\n{}", indent_str(&pretty_hex(&self.data), 4))
    }
}

impl Display for message::ReadDIDReq {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ReadDIDReq(did=0x{:2x})", self.did)
    }
}

impl Display for message::ReadDIDRsp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ReadDIDRsp(did=0x{:02x}):\n{}",
            self.did,
            indent_str(&pretty_hex(&self.user_data), 4)
        )
    }
}

impl Display for message::ReadDTCReq {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ReadDTCReq::{}", self.sub)
    }
}

impl Display for message::ReadDTCRsp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ReadDTCRsp::{}", self.sub)
    }
}

impl Display for message::RequestDownloadReq {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RequestDownloadReq(compress=0x{:1x},_encrypt=0x{:1x}, address=0x{:x}, size=0x{:x})",
            self.compression_method, self.encryption_method, self.memory_address, self.memory_size
        )
    }
}

impl Display for message::RequestDownloadRsp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RequestDownloadRsp() -> max_block_size={}",
            self.max_block_size
        )
    }
}

impl Display for message::TransferDataReq {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TransferDataReq(seq={}, {:02x?})",
            self.block_sequence_counter, self.data
        )
    }
}

impl Display for message::TransferDataRsp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "TransferDataRsp(seq={})", self.block_sequence_counter)
    }
}

impl Display for message::TransferExitReq {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "TransferExitReq({:02x?})", self.user_data)
    }
}

impl Display for message::TransferExitRsp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "TransferExitRsp({:02x?})", self.user_data)
    }
}

impl Display for message::WriteDIDReq {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WriteDIDReq(did=0x{:02x}):\n{}",
            self.did,
            indent_str(&pretty_hex(&self.user_data), 4),
        )
    }
}

impl Display for message::WriteDIDRsp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "WriteDIDReq(did=0x{:2x})", self.did)
    }
}

fn indent_str(astr: &str, indent: usize) -> String {
    let indenter = (1..indent).map(|_| " ").collect::<String>();
    let mut out: String = astr.lines().map(|l| indenter.clone() + l + "\n").collect();
    out.pop();
    out
}
