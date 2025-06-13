#![warn(missing_docs)]
//! UDS message encoder and decoder
//!
//! This crate provides the Unified Diagnostic Services messages, with their
//! associated :
//! - encoding into a writer
//! - decoding from a reader
//!
//! For reference on the protocol, see ISO-13400-2.
//!
//! Unless used directly on top of CAN devices, these messages can we used on an
//! Ethernet based connection, over the `DoIP` protocol as in :
//! [`doip_rw_tokio`](https://crates.io/crates/doip_rw_tokio)
//!
//! A typical reception sequence using the library would be :
//! - call [`uds_read()`]
//!
//! A typical emission sequence using the library would be :
//! - call [`uds_write()`]
mod disp;
mod error;
mod proto;
mod serde;

pub use error::UdsError;

/// Module containing all the *messages* handled by the API.
pub mod message {
    pub use super::proto::did::*;
    pub use super::proto::dtc::*;
    pub use super::proto::nrc::*;
    pub use super::proto::rawuds::*;
    pub use super::proto::transfers::*;
}

use std::{
    fmt::{Display, Formatter},
    io::{Read, Write},
};

#[derive(Debug, Clone)]
/// A UDS message
///
/// The enum is all the message encoded and decoded by this module. If a message
/// is unknown, it is by default encoded or decoded as a `RawUds`.
pub enum UdsMessage {
    /// Negative Response Code
    Nrc(message::Nrc),
    /// `RawUds` is not an actual message, but a placeholder for the UDS message
    /// missing from [`UdsMessage`], which can be encoded as a raw type array.
    RawUds(message::RawUds),
    /// Read DID request message
    ReadDIDReq(message::ReadDIDReq),
    /// Read DID response message
    ReadDIDRsp(message::ReadDIDRsp),
    /// Read DTC request message
    ReadDTCReq(message::ReadDTCReq),
    /// Read DTC response message
    ReadDTCRsp(message::ReadDTCRsp),
    /// Request Download, aka. `TransferStart`
    RequestDownloadReq(message::RequestDownloadReq),
    /// Request Download response
    RequestDownloadRsp(message::RequestDownloadRsp),
    /// Request File Transfer
    RequestFileTransferReq(message::RequestFileTransferReq),
    /// Request File Transfr response
    RequestFileTransferRsp(message::RequestFileTransferRsp),
    /// Transfer data request
    TransferDataReq(message::TransferDataReq),
    /// Transfer data response
    TransferDataRsp(message::TransferDataRsp),
    /// Transfer exit request
    TransferExitReq(message::TransferExitReq),
    /// Transfer exit response
    TransferExitRsp(message::TransferExitRsp),
    /// `WriteDID` request message
    WriteDIDReq(message::WriteDIDReq),
    /// Write DID response message
    WriteDIDRsp(message::WriteDIDRsp),
}

/// Reads a UDS message from a byte stream
///
/// This function is normally called on the diagnostic payload in an `DoIP`
/// message.
///
/// If a message is correctly formed in the input bytes, this function returns
/// [`Ok(UdsMessage)`].
///
/// # Errors
///
/// If the reader returns an error, the error is returned in [`UdsError::Io`].
/// If the input data doesn't decode into a valid uds message, either
/// [`UdsError::PayloadLengthTooShort`] or [`UdsError::UnexpectedPayloadType`]
/// is returned..
///
/// Example:
/// ```
/// use uds_rw::{message, UdsMessage, uds_read};
/// use std::io::Cursor;
///
/// let input = vec![0x62, 0xf1, 0x90, 0x30, 0x39];
/// let message = uds_read(&mut Cursor::new(&input), input.len()).unwrap();
/// if let UdsMessage::ReadDIDRsp(rsp) = message {
///    assert_eq!(rsp.did, 0xf190);
///    assert_eq!(rsp.user_data, vec![0x30, 0x39]);
/// }
/// ```
pub fn uds_read<R: Read>(reader: &mut R, payload_length: usize) -> Result<UdsMessage, UdsError> {
    serde::uds_read(reader, payload_length)
}

/// Writes a UDS message to a writer
///
/// This function is normally called to encode the diagnostic payload in an `DoIP`
/// message.
///
/// If the message can be correctly encoded into the writer, this function
/// returns [`Ok`].
///
/// # Errors
///
/// If the writer returns an error, the error is returned in [`UdsError::Io`].
/// If the output data cannot be encoded into a valid byte stream,
/// [`UdsError::EncodingError`] is returned.
///
/// Example:
/// ```
/// use uds_rw::{message, UdsMessage, uds_write};
///
/// let message = UdsMessage::ReadDIDReq(message::ReadDIDReq { did: 0xf190 });
/// let mut output : Vec<u8> = vec![];
/// uds_write(&mut output, &message).unwrap();
/// assert_eq!(&output, &[0x22, 0xf1, 0x90]);
/// ````
pub fn uds_write<W: Write>(writer: &mut W, msg: &UdsMessage) -> Result<(), UdsError> {
    serde::uds_write(writer, msg)
}

/// Transform a `UdsMessage::RawUds` into a proper Uds message
///
/// If a message is provided in a `RawUds`, it is probable that the API user
/// didn't want to go through the trouble to use the proper `UdsMessage::XYZ`.
/// This function retransforms the `RawUds` into a `ReadDID`,..., or returns the
/// provided `RawUds` if the transformation is not found.
///
/// Always return an `UdsMessage`, which might be the one provided as input
///
/// Example:
/// ```
/// use uds_rw::{message,UdsMessage,uds_rawuds_remove_raw};
///
/// let message = UdsMessage::RawUds(message::RawUds { data: vec![0x19, 0x0a] });
/// let message = uds_rawuds_remove_raw(message);
/// if let UdsMessage::ReadDTCReq(rsp) = message {
///    assert_eq!(rsp.sub, message::DTCReqSubfunction::ReportSupportedDTC);
/// }
/// ```
#[must_use]
pub fn uds_rawuds_remove_raw(msg: UdsMessage) -> UdsMessage {
    if let UdsMessage::RawUds(raw) = &msg {
        let bytes = &raw.data;
        uds_read(&mut std::io::Cursor::new(bytes), bytes.len()).unwrap_or(msg)
    } else {
        msg
    }
}

impl Display for UdsMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        disp::fmt(self, f)
    }
}
