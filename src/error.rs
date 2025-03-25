use thiserror::Error;

#[derive(Error, Debug)]
/// Uds error
pub enum UdsError {
    /// Encoding error, mostly in decoding byte arrays
    #[error("Encoding error: {msg}")]
    EncodingError {
        /// Reason of the encoding error
        msg: String,
    },
    /// Payload too short for this UDS message
    #[error("Payload length in header does match expected payload type length: {value:?}, expected: {expected:?}")]
    PayloadLengthTooShort {
        /// Payload length too short
        value: u32,
        /// Minimal or exact payload length expected
        expected: u32,
    },
    /// Payload unknown, based on the SID
    #[error("Unexpected payload type found: {value:?}")]
    UnexpectedPayloadType {
        /// Unknown SID
        value: u16,
    },
    /// IO error
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
