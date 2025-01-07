#[derive(Clone, Debug, Default)]
/// Request to download a file
pub struct RequestDownloadReq {
    /// Compression method (OEM specific)
    pub compression_method: u8,
    /// Encryption method (OEM specific)
    pub encryption_method: u8,
    /// Size in byte of the `memory_size` field (ie. u8, u16, u32 ...)
    pub memory_size_bytes: u8,
    /// Size in byte of the `memory_address` field (ie. u8, u16, u32 ...)
    pub memory_address_bytes: u8,
    /// Memory address to store the file (OEM specific)
    pub memory_address: usize,
    /// Memory size of the file to store
    pub memory_size: usize,
}

#[derive(Clone, Debug, Default)]
/// Response to a [`RequestDownloadReq`]
pub struct RequestDownloadRsp {
    /// Size in byte of the `max_block_size` field (ie. u8, u16, u32 ...)
    pub max_block_size_bytes: u8,
    /// Maximum size of data in [`TransferDataReq::data`]
    pub max_block_size: usize,
}

#[derive(Clone, Debug, Default)]
/// Transfer data, ie. chunck of the Download or Upload
///
/// The length in `data` is limited by the response in [`RequestDownloadRsp`]
pub struct TransferDataReq {
    /// The sequence block module 256, beginning at 1
    pub block_sequence_counter: u8,
    /// The block content
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
/// Transfer data response
pub struct TransferDataRsp {
    /// The sequence block number acknowledge
    pub block_sequence_counter: u8,
}

#[derive(Clone, Debug, Default)]
/// Transfer exit request, last download/upload request message
pub struct TransferExitReq {
    /// Specific data
    pub user_data: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
/// Transfer exit response, last download/upload request message
pub struct TransferExitRsp {
    /// Specific data
    pub user_data: Vec<u8>,
}
