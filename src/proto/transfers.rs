#[derive(Clone, Debug, Default)]
/// Request to download a file
pub struct RequestDownloadReq {
    /// Compression method (OEM specific)
    pub compression_method: u8,
    /// Encryption method (OEM specific)
    pub encryption_method: u8,
    /// Size in byte of the `memory_size` field (i.e. u8, u16, u32 ...)
    pub memory_size_bytes: u8,
    /// Size in byte of the `memory_address` field (i.e. u8, u16, u32 ...)
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

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
/// Defines the behaviour of `RequestFileTransferReq` operation
pub enum ModeOfOperation {
    /// Reserved for future definition
    #[default]
    Reserved,
    /// Download a file to the server
    AddFile,
    /// Delete a file located in the server
    DeleteFile,
    /// Replace a file located in the server
    ReplaceFile,
    /// Upload a file from the server
    ReadFile,
    /// Read the content of a directory from the server
    ReadDir,
}

#[derive(Clone, Debug, Default)]
/// Request to add/delete/replace/read a file or path (server with file system)
pub struct RequestFileTransferReq {
    /// Type of operation to be applied to `path_name`
    pub mode_of_operation: ModeOfOperation,
    /// Size in byte for the parameter `path_name`
    pub path_bytes: u16,
    /// Location on the server file system
    pub path_name: usize,
    /// Compression method (oem specific)
    pub compression_method: u8,
    /// Encryption method (oem specific)
    pub encryption_method: u8,
    /// Size in byte for parameters `file_size_uncompressed` and `file_size_compressed`
    pub file_size_bytes: u8,
    /// Size in byte of the uncompressed file
    pub file_size_uncompressed: usize,
    /// Size in byte of the compressed file (equal to `file_size_uncompressed` if uncompressed)
    pub file_size_compressed: usize,
}

#[derive(Clone, Debug, Default)]
/// Response to a [`RequestFileTransferReq`]
pub struct RequestFileTransferRsp {
    /// Type of operation applied to the file/dir
    pub mode_of_operation: ModeOfOperation,
    /// Size in byte of the `max_block_size` field (ie. u8, u16, u32 ...)
    pub max_block_size_bytes: u8,
    /// Maximum size of data in [`TransferDataReq::data`]
    pub max_block_size: usize,
    /// Compression method (oem specific)
    pub compression_method: u8,
    /// Encryption method (oem specific)
    pub encryption_method: u8,
    /// Size in byte for the parameters `path_size_uncompressed` and `file_size_compressed`
    pub read_data_size_bytes: u16,
    /// Size in byte of the uncompressed file to be uploaded 
    pub file_size_uncompressed: usize,
    /// Size in byte of the directory information to be read
    pub dir_info_size: usize,
    /// Size in byte of the compressed file
    pub file_size_compressed: usize,
}

#[derive(Clone, Debug, Default)]
/// Transfer data, i.e. chunk of the Download or Upload
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
